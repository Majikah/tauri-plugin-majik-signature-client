//! Native-path streaming commands. These deliberately accept paths rather than FileInput so
//! large files never travel through the webview as base64 or get materialized by FileInput::load.

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::{Duration, Instant};

use majik_signature_client::ipc::dto::{
    resolve_signers, CtxDto, ResolvedTarget, SignDetachedOptionsDto, SignFileOptionsDto,
    VerifyTargetDto,
};
use majik_signature_client::ipc::input::{FileInput, LoadLimits};
use majik_signature_client::logs::HistorySource;
use majik_signature_client::majik_signature::{
    EnvelopeInput, MajikSignatureJSON, StreamingHashProgress,
};
use majik_signature_client::{
    SignDetachedFileOptions, SignFileOptions, SignatureClient, SignatureClientError,
    StreamingDetachedSignResult, StreamingEmbeddedSignResult, StreamingFileOptions, VerifyResult,
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Runtime, State};

use crate::handle::SigHandle;
use crate::system_resources::{RecommendedStreamingConfig, SystemProfile, SystemResourceManager};

type Res<T> = Result<T, SignatureClientError>;

const DEFAULT_CHUNK_SIZE: usize = 1024 * 1024;
const MIN_CHUNK_SIZE: usize = 64 * 1024;
const MAX_CHUNK_SIZE: usize = 128 * 1024 * 1024;
const MAX_ENVELOPE_BYTES: u64 = 16 * 1024 * 1024;
const STREAM_PROGRESS_EVENT_CHANNEL: &str = "majik-signature-client:stream-progress";

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingFileOptionsDto {
    /// Preferred read/write chunk size. Omit or set to null for the current system recommendation.
    pub chunk_size: Option<usize>,
    /// Disable fallback when an explicit chunk-size allocation fails.
    pub strict_chunk_size: Option<bool>,
    /// Optional maximum size for the current file and any prior revision file.
    pub max_bytes: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingEmbeddedSignArgs {
    pub operation_id: String,
    pub input_path: String,
    pub output_path: String,
    #[serde(default)]
    pub options: SignFileOptionsDto,
    #[serde(default)]
    pub ctx: CtxDto,
    #[serde(default)]
    pub streaming: StreamingFileOptionsDto,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingDetachedSignArgs {
    pub operation_id: String,
    pub input_path: String,
    pub output_path: String,
    #[serde(default)]
    pub options: SignDetachedOptionsDto,
    #[serde(default)]
    pub ctx: CtxDto,
    #[serde(default)]
    pub streaming: StreamingFileOptionsDto,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingEmbeddedRevisionArgs {
    pub operation_id: String,
    pub input_path: String,
    pub output_path: String,
    pub prior_file_path: String,
    #[serde(default)]
    pub options: SignFileOptionsDto,
    #[serde(default)]
    pub ctx: CtxDto,
    #[serde(default)]
    pub streaming: StreamingFileOptionsDto,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingDetachedRevisionArgs {
    pub operation_id: String,
    pub input_path: String,
    pub output_path: String,
    pub prior_file_path: String,
    #[serde(default)]
    pub options: SignDetachedOptionsDto,
    #[serde(default)]
    pub ctx: CtxDto,
    #[serde(default)]
    pub streaming: StreamingFileOptionsDto,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingVerifyArgs {
    pub operation_id: String,
    pub input_path: String,
    #[serde(default)]
    pub target: VerifyTargetDto,
    #[serde(default)]
    pub source: Option<HistorySource>,
    #[serde(default)]
    pub streaming: StreamingFileOptionsDto,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingDetachedVerifyArgs {
    pub operation_id: String,
    pub input_path: String,
    pub envelope_path: String,
    #[serde(default)]
    pub target: VerifyTargetDto,
    #[serde(default)]
    pub source: Option<HistorySource>,
    #[serde(default)]
    pub streaming: StreamingFileOptionsDto,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StreamingOutputDto {
    path: String,
    size_bytes: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingSignResultDto {
    signature: MajikSignatureJSON,
    output: StreamingOutputDto,
    source_size_bytes: u64,
    handler: String,
    mime_type: String,
}

impl From<StreamingEmbeddedSignResult> for StreamingSignResultDto {
    fn from(result: StreamingEmbeddedSignResult) -> Self {
        Self {
            signature: result.signature.to_json(),
            output: StreamingOutputDto {
                path: result.output.path,
                size_bytes: result.output.size_bytes,
            },
            source_size_bytes: result.source_size_bytes,
            handler: result.handler,
            mime_type: result.mime_type,
        }
    }
}

impl From<StreamingDetachedSignResult> for StreamingSignResultDto {
    fn from(result: StreamingDetachedSignResult) -> Self {
        Self {
            signature: result.signature.to_json(),
            output: StreamingOutputDto {
                path: result.output.path,
                size_bytes: result.output.size_bytes,
            },
            source_size_bytes: result.source_size_bytes,
            handler: result.handler,
            mime_type: result.mime_type,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct StreamingProgressPayload {
    operation_id: String,
    bytes_processed: u64,
    total_bytes: Option<u64>,
    attempt: usize,
    effective_chunk_size: usize,
    adaptive_fallback: bool,
}

#[derive(Debug, Clone, Serialize)]
struct StreamingProgressEvent {
    #[serde(rename = "type")]
    event_type: &'static str,
    payload: StreamingProgressPayload,
}

fn sign_file_options(client: &SignatureClient, dto: SignFileOptionsDto) -> Res<SignFileOptions> {
    if dto.prior_signed_file.is_some() {
        return Err(SignatureClientError::Invalid(
            "Streaming revisions require priorFilePath; the in-memory priorSignedFile field is not accepted.".into(),
        ));
    }
    let expected_signers = dto
        .expected_signers
        .as_deref()
        .map(|refs| resolve_signers(client, refs))
        .transpose()?;

    Ok(SignFileOptions {
        content_type: dto.content_type,
        timestamp: dto.timestamp,
        mime_type: dto.mime_type,
        expected_signers,
        valid_until: dto.valid_until,
        prior_signed_file: None,
        message: dto.message,
        save_history: dto.save_history,
    })
}

fn detached_file_options(
    client: &SignatureClient,
    dto: SignDetachedOptionsDto,
) -> Res<SignDetachedFileOptions> {
    if dto.prior_file.is_some() {
        return Err(SignatureClientError::Invalid(
            "Streaming revisions require priorFilePath; the in-memory priorFile field is not accepted.".into(),
        ));
    }
    let expected_signers = dto
        .expected_signers
        .as_deref()
        .map(|refs| resolve_signers(client, refs))
        .transpose()?;
    let existing_envelope = match dto.existing_envelope {
        Some(input) => {
            if let FileInput::Bytes { data, .. } = &input {
                // Bound the encoded payload before base64 decoding allocates its byte buffer.
                let max_encoded_bytes = MAX_ENVELOPE_BYTES.div_ceil(3) * 4;
                if data.trim().len() as u64 > max_encoded_bytes {
                    return Err(SignatureClientError::Invalid(
                        "The detached envelope exceeds the 16 MiB streaming input limit.".into(),
                    ));
                }
            }
            let loaded = input.load_with(LoadLimits {
                max_bytes: MAX_ENVELOPE_BYTES,
                allow_empty: false,
            })?;
            Some(EnvelopeInput::Bytes(loaded.bytes))
        }
        None => None,
    };

    Ok(SignDetachedFileOptions {
        content_type: dto.content_type,
        timestamp: dto.timestamp,
        mime_type: dto.mime_type,
        expected_signers,
        valid_until: dto.valid_until,
        existing_envelope,
        prior_file: None,
        message: dto.message,
        tsa: dto.tsa,
        save_history: dto.save_history,
        session_context_hash: dto.session_context_hash,
    })
}

fn run_with_streaming_options<R, T, F>(
    client: &mut SignatureClient,
    app: AppHandle<R>,
    operation_id: String,
    cancellation: Arc<AtomicBool>,
    config: StreamingFileOptionsDto,
    mut operation: F,
) -> Res<T>
where
    R: Runtime,
    F: for<'a> FnMut(&mut SignatureClient, &mut StreamingFileOptions<'a>) -> Res<T>,
{
    let manager = app.state::<SystemResourceManager>();
    let recommended = manager.recommend_streaming_config().chunk_size;
    let requested = config.chunk_size.unwrap_or(recommended);

    if !(MIN_CHUNK_SIZE..=MAX_CHUNK_SIZE).contains(&requested) {
        return Err(SignatureClientError::Invalid(format!(
            "Streaming chunk size must be between {MIN_CHUNK_SIZE} and {MAX_CHUNK_SIZE} bytes"
        )));
    }

    let mut candidates = vec![requested];
    if !config.strict_chunk_size.unwrap_or(false) {
        // On a failed explicit override, retry the machine recommendation first when it is
        // smaller than the override. Then progressively halve until the supported minimum.
        if recommended < requested && !candidates.contains(&recommended) {
            candidates.push(recommended);
        }
        let mut next = *candidates.last().unwrap_or(&requested);
        while next > MIN_CHUNK_SIZE {
            next = (next / 2).max(MIN_CHUNK_SIZE);
            if !candidates.contains(&next) {
                candidates.push(next);
            }
        }
    }

    let mut last_total_bytes = None;
    for (attempt_index, chunk_size) in candidates.iter().copied().enumerate() {
        let adaptive_fallback = attempt_index > 0;
        if adaptive_fallback {
            // Explicitly reset progress so clients can represent a retry instead of showing a
            // percentage from the failed attempt as though it were still current.
            let _ = app.emit(
                STREAM_PROGRESS_EVENT_CHANNEL,
                StreamingProgressEvent {
                    event_type: "stream-progress",
                    payload: StreamingProgressPayload {
                        operation_id: operation_id.clone(),
                        bytes_processed: 0,
                        total_bytes: last_total_bytes,
                        attempt: attempt_index + 1,
                        effective_chunk_size: chunk_size,
                        adaptive_fallback: true,
                    },
                },
            );
        }

        let mut last_emitted_at: Option<Instant> = None;
        let mut last_emitted_bytes = 0u64;
        let mut on_progress = |progress: StreamingHashProgress| {
            last_total_bytes = progress.total_bytes.or(last_total_bytes);
            let interval_elapsed = last_emitted_at
                .as_ref()
                .map(|last| last.elapsed() >= Duration::from_millis(100))
                .unwrap_or(true);
            let enough_bytes =
                progress.bytes_processed.saturating_sub(last_emitted_bytes) >= 8 * 1024 * 1024;
            let completed = progress
                .total_bytes
                .map(|total| progress.bytes_processed >= total)
                .unwrap_or(false);

            if interval_elapsed || enough_bytes || completed {
                let event = StreamingProgressEvent {
                    event_type: "stream-progress",
                    payload: StreamingProgressPayload {
                        operation_id: operation_id.clone(),
                        bytes_processed: progress.bytes_processed,
                        total_bytes: progress.total_bytes,
                        attempt: attempt_index + 1,
                        effective_chunk_size: chunk_size,
                        adaptive_fallback,
                    },
                };
                let _ = app.emit(STREAM_PROGRESS_EVENT_CHANNEL, event);
                last_emitted_at = Some(Instant::now());
                last_emitted_bytes = progress.bytes_processed;
            }
        };

        let mut stream = StreamingFileOptions {
            chunk_size,
            max_bytes: config.max_bytes,
            cancellation: Some(cancellation.as_ref()),
            on_progress: Some(&mut on_progress),
        };

        match operation(client, &mut stream) {
            Err(error) if error.kind() == "memoryAllocation" && attempt_index + 1 < candidates.len() => {
                continue;
            }
            result => return result,
        }
    }

    Err(SignatureClientError::MemoryAllocation {
        message: "Unable to allocate a streaming buffer at the minimum supported chunk size. Close other applications and retry.".into(),
    })
}

#[tauri::command]
pub async fn sign_file_embedded_stream<R: Runtime>(
    state: State<'_, SigHandle>,
    app: AppHandle<R>,
    args: StreamingEmbeddedSignArgs,
) -> Res<StreamingSignResultDto> {
    let StreamingEmbeddedSignArgs {
        operation_id,
        input_path,
        output_path,
        options,
        ctx,
        streaming,
        overwrite,
    } = args;
    let progress_id = operation_id.clone();

    state
        .run_stream(operation_id, move |client, cancellation| {
            let options = sign_file_options(client, options)?;
            let context = ctx.to_ctx();
            run_with_streaming_options(
                client,
                app,
                progress_id,
                cancellation,
                streaming,
                move |client, stream| {
                    client
                        .sign_file_embedded_stream(
                            &input_path,
                            &output_path,
                            &options,
                            &context,
                            stream,
                            overwrite,
                        )
                        .map(StreamingSignResultDto::from)
                },
            )
        })
        .await
}

#[tauri::command]
pub async fn sign_file_detached_stream<R: Runtime>(
    state: State<'_, SigHandle>,
    app: AppHandle<R>,
    args: StreamingDetachedSignArgs,
) -> Res<StreamingSignResultDto> {
    let StreamingDetachedSignArgs {
        operation_id,
        input_path,
        output_path,
        options,
        ctx,
        streaming,
        overwrite,
    } = args;
    let progress_id = operation_id.clone();

    state
        .run_stream(operation_id, move |client, cancellation| {
            let options = detached_file_options(client, options)?;
            let context = ctx.to_ctx();
            run_with_streaming_options(
                client,
                app,
                progress_id,
                cancellation,
                streaming,
                move |client, stream| {
                    client
                        .sign_file_detached_stream(
                            &input_path,
                            &output_path,
                            &options,
                            &context,
                            stream,
                            overwrite,
                        )
                        .map(StreamingSignResultDto::from)
                },
            )
        })
        .await
}

#[tauri::command]
pub async fn sign_file_embedded_stream_with_prior<R: Runtime>(
    state: State<'_, SigHandle>,
    app: AppHandle<R>,
    args: StreamingEmbeddedRevisionArgs,
) -> Res<StreamingSignResultDto> {
    let StreamingEmbeddedRevisionArgs {
        operation_id,
        input_path,
        output_path,
        prior_file_path,
        options,
        ctx,
        streaming,
        overwrite,
    } = args;
    let progress_id = operation_id.clone();

    state
        .run_stream(operation_id, move |client, cancellation| {
            let options = sign_file_options(client, options)?;
            let context = ctx.to_ctx();
            run_with_streaming_options(
                client,
                app,
                progress_id,
                cancellation,
                streaming,
                move |client, stream| {
                    client
                        .sign_file_embedded_stream_with_prior(
                            &input_path,
                            &output_path,
                            &prior_file_path,
                            &options,
                            &context,
                            stream,
                            overwrite,
                        )
                        .map(StreamingSignResultDto::from)
                },
            )
        })
        .await
}

#[tauri::command]
pub async fn sign_file_detached_stream_with_prior<R: Runtime>(
    state: State<'_, SigHandle>,
    app: AppHandle<R>,
    args: StreamingDetachedRevisionArgs,
) -> Res<StreamingSignResultDto> {
    let StreamingDetachedRevisionArgs {
        operation_id,
        input_path,
        output_path,
        prior_file_path,
        options,
        ctx,
        streaming,
        overwrite,
    } = args;
    let progress_id = operation_id.clone();

    state
        .run_stream(operation_id, move |client, cancellation| {
            let options = detached_file_options(client, options)?;
            let context = ctx.to_ctx();
            run_with_streaming_options(
                client,
                app,
                progress_id,
                cancellation,
                streaming,
                move |client, stream| {
                    client
                        .sign_file_detached_stream_with_prior(
                            &input_path,
                            &output_path,
                            &prior_file_path,
                            &options,
                            &context,
                            stream,
                            overwrite,
                        )
                        .map(StreamingSignResultDto::from)
                },
            )
        })
        .await
}

#[tauri::command]
pub async fn verify_file_embedded_stream<R: Runtime>(
    state: State<'_, SigHandle>,
    app: AppHandle<R>,
    args: StreamingVerifyArgs,
) -> Res<VerifyResult> {
    let StreamingVerifyArgs {
        operation_id,
        input_path,
        target,
        source,
        streaming,
    } = args;
    let progress_id = operation_id.clone();

    state
        .run_stream(operation_id, move |client, cancellation| {
            let target = ResolvedTarget::new(client, &target)?;
            run_with_streaming_options(
                client,
                app,
                progress_id,
                cancellation,
                streaming,
                move |client, stream| {
                    client.verify_file_embedded_stream(
                        &input_path,
                        &target.target(None),
                        stream,
                        source.clone().unwrap_or(HistorySource::Gui),
                    )
                },
            )
        })
        .await
}

#[tauri::command]
pub async fn verify_file_detached_stream<R: Runtime>(
    state: State<'_, SigHandle>,
    app: AppHandle<R>,
    args: StreamingDetachedVerifyArgs,
) -> Res<VerifyResult> {
    let StreamingDetachedVerifyArgs {
        operation_id,
        input_path,
        envelope_path,
        target,
        source,
        streaming,
    } = args;
    let progress_id = operation_id.clone();

    state
        .run_stream(operation_id, move |client, cancellation| {
            let target = ResolvedTarget::new(client, &target)?;
            run_with_streaming_options(
                client,
                app,
                progress_id,
                cancellation,
                streaming,
                move |client, stream| {
                    client.verify_file_detached_stream(
                        &input_path,
                        &envelope_path,
                        &target.target(None),
                        stream,
                        source.unwrap_or(HistorySource::Gui),
                    )
                },
            )
        })
        .await
}

#[tauri::command]
pub fn cancel_stream(state: State<'_, SigHandle>, operation_id: String) -> bool {
    state.cancel_stream(&operation_id)
}


#[tauri::command]
pub fn get_system_profile(resources: State<'_, SystemResourceManager>) -> SystemProfile {
    resources.inspect_system()
}

#[tauri::command]
pub fn get_recommended_streaming_config(
    resources: State<'_, SystemResourceManager>,
) -> RecommendedStreamingConfig {
    resources.recommend_streaming_config()
}
