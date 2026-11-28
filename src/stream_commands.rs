//! Native-path streaming commands. These deliberately accept paths rather than FileInput so
//! large files never travel through the webview as base64 or get materialized by FileInput::load.

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::{Duration, Instant};

use majik_signature_client::ipc::dto::{
    resolve_signers, BatchSummaryDto, BatchModeDto, CtxDto, FailureDto, FileVerifyDto,
    MapVerifyDto, ResolvedTarget, SignDetachedOptionsDto, SignFileOptionsDto, VerifyTargetDto,
};
use majik_signature_client::ipc::input::{FileInput, LoadLimits};
use majik_signature_client::logs::HistorySource;
use majik_signature_client::majik_signature::{
    BatchSignMode, BatchSignOptions, BatchSignResult, EnvelopeInput, MajikSignatureJSON,
    StreamingHashProgress,
};
use majik_signature_client::{
    SignDetachedFileOptions, SignFileOptions, SignatureClient, SignatureClientError,
    write_atomic_bytes, BatchStreamingFileOptions, StreamingBatchPathInput, StreamingBatchProgress,
    StreamingDetachedSignResult, StreamingEmbeddedSignResult, StreamingFileOptions, StreamingOutput,
    VerifyResult,
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Runtime, State};

use crate::handle::SigHandle;
use crate::system_resources::{RecommendedStreamingConfig, SystemProfile, SystemResourceManager};

type Res<T> = Result<T, SignatureClientError>;

/// All limits are revalidated in Rust even when the TypeScript facade checks first.
const MIN_CHUNK_SIZE: usize = 64 * 1024;
/// Matches the Rust client 1.6.0 streaming implementation.
const MAX_CHUNK_SIZE: usize = 64 * 1024 * 1024;
const MAX_ENVELOPE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_MAP_BYTES: u64 = 64 * 1024 * 1024;
const MAX_NATIVE_PATH_CHARS: usize = 4_096;
const MAX_OPERATION_ID_BYTES: usize = 128;
const MAX_STREAMING_BATCH_FILES: usize = 10_000;
const MAX_BATCH_METADATA_BYTES: usize = 8 * 1024 * 1024;
const MAX_BATCH_EXPECTED_SIGNERS: usize = 1_000;
const STREAM_PROGRESS_EVENT_CHANNEL: &str = "majik-signature-client:stream-progress";

/// Wire-level stream controls validated before native paths are opened.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    file_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_files: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct StreamingProgressEvent {
    #[serde(rename = "type")]
    event_type: &'static str,
    payload: StreamingProgressPayload,
}


/// Validate bounded text values before they are processed by native code.
fn validate_bounded_text(value: &str, field: &str, max_utf16: usize) -> Res<()> {
    if value.encode_utf16().count() > max_utf16
        || value.chars().any(|character| {
            matches!(
                character,
                '\u{0000}'..='\u{001f}'
                    | '\u{007f}'
                    | '\u{202a}'..='\u{202e}'
                    | '\u{2066}'..='\u{2069}'
            )
        })
    {
        return Err(SignatureClientError::Invalid(format!(
            "{field} exceeds its supported length or contains unsafe characters"
        )));
    }
    Ok(())
}

/// Validate a native filesystem path without introducing a path-sandbox policy. The host app's
/// Tauri capabilities and OS permissions remain the authority for filesystem access.
fn validate_native_path(path: &str, field: &str) -> Res<()> {
    if path.trim().is_empty() {
        return Err(SignatureClientError::Invalid(format!("{field} must not be empty")));
    }
    validate_bounded_text(path, field, MAX_NATIVE_PATH_CHARS)
}

/// Operation IDs are correlation tokens and must remain small, ASCII and predictable.
fn validate_operation_id(id: &str) -> Res<()> {
    if id.is_empty()
        || id.len() > MAX_OPERATION_ID_BYTES
        || !id.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':')
        })
    {
        return Err(SignatureClientError::Invalid(
            "Streaming operationId must be 1–128 restricted ASCII bytes".into(),
        ));
    }
    Ok(())
}

fn validate_streaming_options(options: &StreamingFileOptionsDto) -> Res<()> {
    if let Some(size) = options.chunk_size {
        if !(MIN_CHUNK_SIZE..=MAX_CHUNK_SIZE).contains(&size) {
            return Err(SignatureClientError::Invalid(format!(
                "Streaming chunk size must be between {MIN_CHUNK_SIZE} and {MAX_CHUNK_SIZE} bytes"
            )));
        }
    }
    Ok(())
}

fn validate_signing_metadata(
    content_type: Option<&str>,
    timestamp: Option<&str>,
    mime_type: Option<&str>,
    valid_until: Option<&str>,
    message: Option<&str>,
    expected_signers: usize,
) -> Res<()> {
    for (value, field, limit) in [
        (content_type, "contentType", 1_024usize),
        (timestamp, "timestamp", 128usize),
        (mime_type, "mimeType", 1_024usize),
        (valid_until, "validUntil", 128usize),
        (message, "message", 4_096usize),
    ] {
        if let Some(value) = value {
            validate_bounded_text(value, field, limit)?;
        }
    }
    if expected_signers > MAX_BATCH_EXPECTED_SIGNERS {
        return Err(SignatureClientError::Invalid(format!(
            "expectedSigners exceeds the {MAX_BATCH_EXPECTED_SIGNERS}-entry limit"
        )));
    }
    Ok(())
}

fn validate_signer_refs(refs: Option<&[majik_signature_client::ipc::dto::SignerRefDto]>) -> Res<()> {
    let Some(refs) = refs else { return Ok(()); };
    if refs.len() > MAX_BATCH_EXPECTED_SIGNERS {
        return Err(SignatureClientError::Invalid(format!(
            "expectedSigners exceeds the {MAX_BATCH_EXPECTED_SIGNERS}-entry limit"
        )));
    }
    for item in refs {
        match item {
            majik_signature_client::ipc::dto::SignerRefDto::Contact { contact_id } => {
                validate_bounded_text(contact_id, "expectedSigners.contactId", 256)?;
            }
            majik_signature_client::ipc::dto::SignerRefDto::Account { account_id } => {
                validate_bounded_text(account_id, "expectedSigners.accountId", 256)?;
            }
            majik_signature_client::ipc::dto::SignerRefDto::Expected(expected) => {
                validate_bounded_text(&expected.signer_id, "expectedSigners.signerId", 256)?;
                validate_bounded_text(&expected.ed_public_key, "expectedSigners.edPublicKey", 1_024)?;
                validate_bounded_text(&expected.ml_dsa_public_key, "expectedSigners.mlDsaPublicKey", 8_192)?;
            }
        }
    }
    Ok(())
}

fn validate_file_input_metadata(input: &FileInput, field: &str) -> Res<()> {
    match input {
        FileInput::Path { path, mime_type } => {
            validate_native_path(path, field)?;
            if let Some(mime) = mime_type {
                validate_bounded_text(mime, "mimeType", 1_024)?;
            }
        }
        FileInput::Bytes { data, name, mime_type } => {
            let max_encoded = MAX_ENVELOPE_BYTES.div_ceil(3) * 4;
            if data.trim().len() as u64 > max_encoded {
                return Err(SignatureClientError::Invalid(format!(
                    "{field} exceeds the 16 MiB inline envelope limit"
                )));
            }
            if let Some(name) = name {
                validate_bounded_text(name, "fileName", MAX_NATIVE_PATH_CHARS)?;
            }
            if let Some(mime) = mime_type {
                validate_bounded_text(mime, "mimeType", 1_024)?;
            }
        }
    }
    Ok(())
}

fn validate_sign_file_dto(dto: &SignFileOptionsDto) -> Res<()> {
    validate_signing_metadata(
        dto.content_type.as_deref(),
        dto.timestamp.as_deref(),
        dto.mime_type.as_deref(),
        dto.valid_until.as_deref(),
        dto.message.as_deref(),
        dto.expected_signers.as_ref().map_or(0, Vec::len),
    )?;
    validate_signer_refs(dto.expected_signers.as_deref())?;
    if let Some(prior) = dto.prior_signed_file.as_ref() {
        validate_file_input_metadata(prior, "priorSignedFile")?;
    }
    Ok(())
}

fn validate_detached_sign_dto(dto: &SignDetachedOptionsDto) -> Res<()> {
    validate_signing_metadata(
        dto.content_type.as_deref(),
        dto.timestamp.as_deref(),
        dto.mime_type.as_deref(),
        dto.valid_until.as_deref(),
        dto.message.as_deref(),
        dto.expected_signers.as_ref().map_or(0, Vec::len),
    )?;
    validate_signer_refs(dto.expected_signers.as_deref())?;
    if let Some(envelope) = dto.existing_envelope.as_ref() {
        validate_file_input_metadata(envelope, "existingEnvelope")?;
    }
    if let Some(prior) = dto.prior_file.as_ref() {
        validate_file_input_metadata(prior, "priorFile")?;
    }
    if let Some(hash) = dto.session_context_hash.as_deref() {
        validate_bounded_text(hash, "sessionContextHash", 256)?;
    }
    Ok(())
}

fn validate_verification_target(target: &VerifyTargetDto) -> Res<()> {
    for (value, field, limit) in [
        (target.contact_id.as_deref(), "contactId", 256usize),
        (target.address.as_deref(), "address", 4_096usize),
        (target.account_id.as_deref(), "accountId", 256usize),
        (target.expected_signer_id.as_deref(), "expectedSignerId", 256usize),
        (target.mime_type.as_deref(), "mimeType", 1_024usize),
        (target.now.as_deref(), "now", 128usize),
    ] {
        if let Some(value) = value {
            validate_bounded_text(value, field, limit)?;
        }
    }
    Ok(())
}

fn sign_file_options(client: &SignatureClient, dto: SignFileOptionsDto) -> Res<SignFileOptions> {
    validate_sign_file_dto(&dto)?;
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
    validate_detached_sign_dto(&dto)?;
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
            validate_file_input_metadata(&input, "existingEnvelope")?;
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
    validate_streaming_options(&config)?;
    let manager = app.state::<SystemResourceManager>();
    let recommended = manager.recommend_streaming_config().chunk_size;
    let requested = config.chunk_size.unwrap_or(recommended);

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
                        file_index: None,
                        total_files: None,
                        path: None,
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
                        file_index: None,
                        total_files: None,
                        path: None,
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
            Err(error)
                if error.kind() == "memoryAllocation" && attempt_index + 1 < candidates.len() =>
            {
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
    validate_operation_id(&operation_id)?;
    validate_native_path(&input_path, "inputPath")?;
    validate_native_path(&output_path, "outputPath")?;
    validate_streaming_options(&streaming)?;
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
    validate_operation_id(&operation_id)?;
    validate_native_path(&input_path, "inputPath")?;
    validate_native_path(&output_path, "outputPath")?;
    validate_streaming_options(&streaming)?;
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
        operation_id, input_path, output_path, prior_file_path, options, ctx, streaming, overwrite,
    } = args;
    validate_operation_id(&operation_id)?;
    validate_native_path(&input_path, "inputPath")?;
    validate_native_path(&output_path, "outputPath")?;
    validate_native_path(&prior_file_path, "priorFilePath")?;
    validate_streaming_options(&streaming)?;
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
        operation_id, input_path, output_path, prior_file_path, options, ctx, streaming, overwrite,
    } = args;
    validate_operation_id(&operation_id)?;
    validate_native_path(&input_path, "inputPath")?;
    validate_native_path(&output_path, "outputPath")?;
    validate_native_path(&prior_file_path, "priorFilePath")?;
    validate_streaming_options(&streaming)?;
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
    let StreamingVerifyArgs { operation_id, input_path, target, source, streaming } = args;
    validate_operation_id(&operation_id)?;
    validate_native_path(&input_path, "inputPath")?;
    validate_verification_target(&target)?;
    validate_streaming_options(&streaming)?;
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
                        source.unwrap_or(HistorySource::Gui),
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
    let StreamingDetachedVerifyArgs { operation_id, input_path, envelope_path, target, source, streaming } = args;
    validate_operation_id(&operation_id)?;
    validate_native_path(&input_path, "inputPath")?;
    validate_native_path(&envelope_path, "envelopePath")?;
    validate_verification_target(&target)?;
    validate_streaming_options(&streaming)?;
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


/// Native source path paired with the relative path stored in a signature map.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingBatchPathInputDto {
    pub manifest_path: String,
    pub source_path: String,
    #[serde(default)]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchStreamingFileOptionsDto {
    pub chunk_size: Option<usize>,
    pub max_bytes_per_file: Option<u64>,
    pub strict_chunk_size: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingBatchSignArgs {
    pub operation_id: String,
    pub inputs: Vec<StreamingBatchPathInputDto>,
    #[serde(default)]
    pub options: BatchDetachedOptionsWireDto,
    #[serde(default)]
    pub ctx: Option<CtxDto>,
    #[serde(default)]
    pub streaming: Option<BatchStreamingFileOptionsDto>,
    pub map_output_path: Option<String>,
    pub output_dir: Option<String>,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDetachedOptionsWireDto {
    pub mode: Option<BatchModeDto>,
    pub content_type: Option<String>,
    pub timestamp: Option<String>,
    pub expected_signers: Option<Vec<majik_signature_client::ipc::dto::SignerRefDto>>,
    pub valid_until: Option<String>,
    #[serde(default)]
    pub continue_on_error: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingBatchVerifyArgs {
    pub operation_id: String,
    pub map_path: String,
    pub inputs: Vec<StreamingBatchPathInputDto>,
    #[serde(default)]
    pub target: VerifyTargetDto,
    #[serde(default)]
    pub require_all_present: bool,
    #[serde(default)]
    pub source: Option<HistorySource>,
    #[serde(default)]
    pub streaming: Option<BatchStreamingFileOptionsDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BatchStreamingOutputDto {
    path: String,
    size_bytes: u64,
}

impl From<StreamingOutput> for BatchStreamingOutputDto {
    fn from(output: StreamingOutput) -> Self {
        Self { path: output.path, size_bytes: output.size_bytes }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BatchStreamingSignatureOutputDto {
    manifest_path: String,
    output: BatchStreamingOutputDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingBatchSignResultDto {
    mode: String,
    map: Option<BatchStreamingOutputDto>,
    signatures: Vec<BatchStreamingSignatureOutputDto>,
    failures: Vec<FailureDto>,
}

fn validate_batch_inputs(inputs: &[StreamingBatchPathInputDto]) -> Res<()> {
    if inputs.is_empty() || inputs.len() > MAX_STREAMING_BATCH_FILES {
        return Err(SignatureClientError::Invalid(format!(
            "Streaming batch must contain 1–{MAX_STREAMING_BATCH_FILES} files"
        )));
    }
    let mut total_metadata_bytes = 0usize;
    for input in inputs {
        validate_bounded_text(&input.manifest_path, "manifestPath", 4_096)?;
        validate_native_path(&input.source_path, "sourcePath")?;
        if let Some(mime) = input.mime_type.as_deref() {
            validate_bounded_text(mime, "mimeType", 1_024)?;
        }
        total_metadata_bytes = total_metadata_bytes
            .saturating_add(input.manifest_path.len())
            .saturating_add(input.source_path.len())
            .saturating_add(input.mime_type.as_ref().map_or(0, String::len));
        if total_metadata_bytes > MAX_BATCH_METADATA_BYTES {
            return Err(SignatureClientError::Invalid(
                "Streaming batch metadata exceeds the 8 MiB request limit".into(),
            ));
        }
    }
    Ok(())
}

fn validate_batch_streaming_options(options: &BatchStreamingFileOptionsDto) -> Res<()> {
    let chunk = options.chunk_size.unwrap_or(1024 * 1024);
    if !(MIN_CHUNK_SIZE..=MAX_CHUNK_SIZE).contains(&chunk) {
        return Err(SignatureClientError::Invalid(format!(
            "Streaming batch chunk size must be between {MIN_CHUNK_SIZE} and {MAX_CHUNK_SIZE} bytes"
        )));
    }
    Ok(())
}

fn read_mjksmap_bounded(path: &str) -> Res<Vec<u8>> {
    validate_native_path(path, "mapPath")?;
    // Inspect the path before opening it so directories, devices and FIFOs are rejected early.
    let path_metadata = std::fs::metadata(path)
        .map_err(|error| SignatureClientError::Invalid(format!("Unable to inspect MJKSMAP path: {error}")))?;
    if !path_metadata.is_file() || path_metadata.len() == 0 || path_metadata.len() > MAX_MAP_BYTES {
        return Err(SignatureClientError::Invalid(
            "MJKSMAP input must be a non-empty regular file no larger than 64 MiB".into(),
        ));
    }
    let file = File::open(path)
        .map_err(|error| SignatureClientError::Invalid(format!("Unable to open MJKSMAP: {error}")))?;
    let metadata = file.metadata()
        .map_err(|error| SignatureClientError::Invalid(format!("Unable to inspect opened MJKSMAP: {error}")))?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > MAX_MAP_BYTES {
        return Err(SignatureClientError::Invalid(
            "MJKSMAP input must be a non-empty regular file no larger than 64 MiB".into(),
        ));
    }
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(metadata.len() as usize).map_err(|error| {
        SignatureClientError::MemoryAllocation {
            message: format!("Unable to reserve the bounded MJKSMAP buffer: {error}"),
        }
    })?;
    file.take(MAX_MAP_BYTES + 1).read_to_end(&mut bytes)
        .map_err(|error| SignatureClientError::Invalid(format!("Unable to read MJKSMAP: {error}")))?;
    if bytes.is_empty() || bytes.len() as u64 > MAX_MAP_BYTES {
        return Err(SignatureClientError::Invalid(
            "MJKSMAP input changed while reading or exceeds the 64 MiB limit".into(),
        ));
    }
    Ok(bytes)
}

fn batch_inputs(inputs: &[StreamingBatchPathInputDto]) -> Vec<StreamingBatchPathInput> {
    inputs.iter().map(|input| StreamingBatchPathInput {
        manifest_path: input.manifest_path.clone(),
        source_path: PathBuf::from(&input.source_path),
        mime_type: input.mime_type.clone(),
    }).collect()
}

fn run_with_batch_streaming_options<R, T, F>(
    client: &mut SignatureClient,
    app: &AppHandle<R>,
    operation_id: String,
    cancellation: Arc<AtomicBool>,
    config: BatchStreamingFileOptionsDto,
    mut operation: F,
) -> Res<T>
where
    R: Runtime,
    F: for<'a> FnMut(&mut SignatureClient, &mut BatchStreamingFileOptions<'a>) -> Res<T>,
{
    validate_batch_streaming_options(&config)?;
    let recommended = app.state::<SystemResourceManager>().recommend_streaming_config().chunk_size;
    let requested = config.chunk_size.unwrap_or(recommended);
    let mut candidates = vec![requested];
    if !config.strict_chunk_size.unwrap_or(false) {
        if recommended < requested { candidates.push(recommended); }
        let mut next = *candidates.last().unwrap_or(&requested);
        while next > MIN_CHUNK_SIZE {
            next = (next / 2).max(MIN_CHUNK_SIZE);
            if !candidates.contains(&next) { candidates.push(next); }
        }
    }
    let mut previous_progress: Option<(usize, usize, String, Option<u64>)> = None;
    for (attempt, chunk_size) in candidates.iter().copied().enumerate() {
        let fallback = attempt > 0;
        if let Some((index, total, path, total_bytes)) = previous_progress.as_ref() {
            let _ = app.emit(STREAM_PROGRESS_EVENT_CHANNEL, StreamingProgressEvent {
                event_type: "stream-progress",
                payload: StreamingProgressPayload {
                    operation_id: operation_id.clone(),
                    bytes_processed: 0,
                    total_bytes: *total_bytes,
                    attempt: attempt + 1,
                    effective_chunk_size: chunk_size,
                    adaptive_fallback: true,
                    file_index: Some(*index),
                    total_files: Some(*total),
                    path: Some(path.clone()),
                },
            });
        }
        let mut on_progress = |progress: StreamingBatchProgress| {
            previous_progress = Some((
                progress.index,
                progress.total_files,
                progress.path.clone(),
                progress.total_bytes,
            ));
            let _ = app.emit(STREAM_PROGRESS_EVENT_CHANNEL, StreamingProgressEvent {
                event_type: "stream-progress",
                payload: StreamingProgressPayload {
                    operation_id: operation_id.clone(),
                    bytes_processed: progress.bytes_processed,
                    total_bytes: progress.total_bytes,
                    attempt: attempt + 1,
                    effective_chunk_size: chunk_size,
                    adaptive_fallback: fallback,
                    file_index: Some(progress.index),
                    total_files: Some(progress.total_files),
                    path: Some(progress.path),
                },
            });
        };
        let mut stream_options = BatchStreamingFileOptions {
            chunk_size,
            max_bytes_per_file: config.max_bytes_per_file,
            cancellation: Some(cancellation.as_ref()),
            on_progress: Some(&mut on_progress),
        };
        match operation(client, &mut stream_options) {
            Err(error) if error.kind() == "memoryAllocation" && attempt + 1 < candidates.len() => continue,
            result => return result,
        }
    }
    Err(SignatureClientError::MemoryAllocation {
        message: "Unable to allocate a streaming batch buffer at the minimum supported chunk size.".into(),
    })
}

/// Stream-sign native files into one MJKSMAP file or separate .mjksig sidecars.
#[tauri::command]
pub async fn sign_batch_detached_paths_stream<R: Runtime>(
    state: State<'_, SigHandle>,
    app: AppHandle<R>,
    args: StreamingBatchSignArgs,
) -> Res<StreamingBatchSignResultDto> {
    validate_operation_id(&args.operation_id)?;
    validate_batch_inputs(&args.inputs)?;
    let streaming = args.streaming.clone().unwrap_or_default();
    validate_batch_streaming_options(&streaming)?;
    validate_signing_metadata(
        args.options.content_type.as_deref(),
        args.options.timestamp.as_deref(),
        None,
        args.options.valid_until.as_deref(),
        None,
        args.options.expected_signers.as_ref().map_or(0, Vec::len),
    )?;
    validate_signer_refs(args.options.expected_signers.as_deref())?;
    match args.options.mode.unwrap_or(BatchModeDto::Map) {
        BatchModeDto::Map => validate_native_path(args.map_output_path.as_deref().unwrap_or(""), "mapOutputPath")?,
        BatchModeDto::Separate => {
            let output_dir = args.output_dir.as_deref().unwrap_or("");
            validate_native_path(output_dir, "outputDir")?;
            if !Path::new(output_dir).is_dir() {
                return Err(SignatureClientError::Invalid("outputDir must be an existing directory".into()));
            }
        }
    }
    let operation_id = args.operation_id.clone();
    let inputs = batch_inputs(&args.inputs);
    let wire_options = args.options;
    let context = args.ctx.unwrap_or_default().to_ctx();
    let map_output_path = args.map_output_path;
    let output_dir = args.output_dir;
    let overwrite = args.overwrite;
    let progress_id = operation_id.clone();
    state.run_stream(operation_id, move |client, cancellation| {
        let expected_signers = wire_options.expected_signers.as_deref()
            .map(|refs| resolve_signers(client, refs)).transpose()?;
        let batch_options = BatchSignOptions {
            content_type: wire_options.content_type.clone(),
            timestamp: wire_options.timestamp.clone(),
            expected_signers,
            valid_until: wire_options.valid_until.clone(),
            mode: Some(match wire_options.mode.unwrap_or(BatchModeDto::Map) {
                BatchModeDto::Map => BatchSignMode::Map,
                BatchModeDto::Separate => BatchSignMode::Separate,
            }),
            continue_on_error: wire_options.continue_on_error,
        };
        run_with_batch_streaming_options(
            client, &app, progress_id, cancellation, streaming,
            move |client, stream_options| {
                match client.sign_batch_detached_paths_stream(&inputs, &batch_options, &context, stream_options)? {
                    BatchSignResult::Map { map_blob, failures, .. } => {
                        let output = write_atomic_bytes(
                            map_output_path.as_deref().expect("validated mapOutputPath"),
                            &map_blob,
                            overwrite,
                        )?;
                        Ok(StreamingBatchSignResultDto {
                            mode: "map".into(),
                            map: Some(output.into()),
                            signatures: Vec::new(),
                            failures: failures.into_iter().map(|failure| FailureDto {
                                path: failure.path, error: failure.error
                            }).collect(),
                        })
                    }
                    BatchSignResult::Separate { signatures, failures } => {
                        let folder = Path::new(output_dir.as_deref().expect("validated outputDir"));
                        let mut outputs = Vec::with_capacity(signatures.len());
                        for (index, signature) in signatures.into_iter().enumerate() {
                            let leaf = Path::new(&signature.path).file_name()
                                .and_then(|name| name.to_str()).unwrap_or("signature");
                            let path = folder.join(format!("{:05}-{leaf}.mjksig", index + 1));
                            let output = write_atomic_bytes(path, &signature.blob, overwrite)?;
                            outputs.push(BatchStreamingSignatureOutputDto {
                                manifest_path: signature.path,
                                output: output.into(),
                            });
                        }
                        Ok(StreamingBatchSignResultDto {
                            mode: "separate".into(),
                            map: None,
                            signatures: outputs,
                            failures: failures.into_iter().map(|failure| FailureDto {
                                path: failure.path, error: failure.error
                            }).collect(),
                        })
                    }
                }
            },
        )
    }).await
}

/// Verify native files against an MJKSMAP file with a trusted signer target.
#[tauri::command]
pub async fn verify_mjks_map_paths_stream<R: Runtime>(
    state: State<'_, SigHandle>,
    app: AppHandle<R>,
    args: StreamingBatchVerifyArgs,
) -> Res<MapVerifyDto> {
    validate_operation_id(&args.operation_id)?;
    validate_native_path(&args.map_path, "mapPath")?;
    validate_batch_inputs(&args.inputs)?;
    let streaming = args.streaming.clone().unwrap_or_default();
    validate_batch_streaming_options(&streaming)?;
    validate_verification_target(&args.target)?;
    if args.target.account_id.is_none() && args.target.contact_id.is_none() && args.target.address.is_none() {
        return Err(SignatureClientError::Invalid(
            "Streaming MJKSMAP verification requires a trusted account, contact or address".into(),
        ));
    }
    let inputs = batch_inputs(&args.inputs);
    let target = args.target;
    let source = args.source.unwrap_or(HistorySource::Gui);
    let require_all_present = args.require_all_present;
    let map_path = args.map_path;
    let operation_id = args.operation_id.clone();
    let progress_id = operation_id.clone();
    state.run_stream(operation_id, move |client, cancellation| {
        let map_bytes = read_mjksmap_bounded(&map_path)?;
        let resolved = ResolvedTarget::new(client, &target)?;
        run_with_batch_streaming_options(
            client, &app, progress_id, cancellation, streaming,
            move |client, stream_options| {
                let result = client.verify_mjks_map_paths_stream(
                    majik_signature_client::MjksMapInput::Bytes(map_bytes.clone()),
                    &inputs,
                    &resolved.target(None),
                    require_all_present,
                    source,
                    stream_options,
                )?;
                Ok(MapVerifyDto {
                    results: result.results.into_iter().map(|entry| FileVerifyDto {
                        path: entry.path,
                        status: entry.status,
                        results: entry.results,
                        reason: entry.reason,
                        relocated_from: entry.relocated_from,
                    }).collect(),
                    summary: BatchSummaryDto {
                        total: result.summary.total,
                        verified: result.summary.verified,
                        invalid: result.summary.invalid,
                        tampered: result.summary.tampered,
                        not_in_map: result.summary.not_in_map,
                        all_valid: result.summary.all_valid,
                    },
                    missing_from_batch: result.missing_from_batch,
                })
            },
        )
    }).await
}

#[tauri::command]
pub fn cancel_stream(state: State<'_, SigHandle>, operation_id: String) -> bool {
    validate_operation_id(&operation_id).is_ok() && state.cancel_stream(&operation_id)
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


#[cfg(test)]
mod boundary_tests {
    use super::*;

    #[test]
    fn rejects_empty_overlong_and_unsafe_native_paths() {
        assert!(validate_native_path("", "inputPath").is_err());
        assert!(validate_native_path(&"a".repeat(MAX_NATIVE_PATH_CHARS + 1), "inputPath").is_err());
        assert!(validate_native_path("bad\0path", "inputPath").is_err());
        assert!(validate_native_path("bad\u{202e}path", "inputPath").is_err());
        assert!(validate_native_path("folder/document.pdf", "inputPath").is_ok());
    }

    #[test]
    fn rejects_bad_operation_ids_and_chunks_outside_core_limits() {
        assert!(validate_operation_id("").is_err());
        assert!(validate_operation_id(&"x".repeat(MAX_OPERATION_ID_BYTES + 1)).is_err());
        assert!(validate_operation_id("op:1234-abc_1").is_ok());
        assert!(validate_operation_id("bad\nid").is_err());
        assert!(validate_streaming_options(&StreamingFileOptionsDto {
            chunk_size: Some(MAX_CHUNK_SIZE + 1),
            ..Default::default()
        }).is_err());
        assert!(validate_streaming_options(&StreamingFileOptionsDto {
            chunk_size: Some(MIN_CHUNK_SIZE),
            ..Default::default()
        }).is_ok());
        assert!(validate_batch_streaming_options(&BatchStreamingFileOptionsDto {
            chunk_size: Some(MAX_CHUNK_SIZE + 1),
            ..Default::default()
        }).is_err());
    }

    #[test]
    fn rejects_batch_overflow_and_long_or_malicious_metadata_before_opening_sources() {
        let sample = StreamingBatchPathInputDto {
            manifest_path: "folder/file.bin".into(),
            source_path: "not-opened.bin".into(),
            mime_type: None,
        };
        assert!(validate_batch_inputs(&[]).is_err());
        assert!(validate_batch_inputs(&vec![sample.clone(); MAX_STREAMING_BATCH_FILES + 1]).is_err());
        assert!(validate_batch_inputs(&[StreamingBatchPathInputDto {
            source_path: "x".repeat(MAX_NATIVE_PATH_CHARS + 1),
            ..sample.clone()
        }]).is_err());
        assert!(validate_batch_inputs(&[StreamingBatchPathInputDto {
            manifest_path: "bad\u{202e}name.pdf".into(),
            ..sample.clone()
        }]).is_err());

        let oversized_metadata = vec![StreamingBatchPathInputDto {
            manifest_path: "m".repeat(4_096),
            source_path: "s".repeat(4_096),
            mime_type: None,
        }; 1_100];
        assert!(validate_batch_inputs(&oversized_metadata).is_err());
    }

    #[test]
    fn rejects_large_signing_metadata_and_inline_envelopes_before_decode() {
        assert!(validate_bounded_text(&"m".repeat(1_025), "mimeType", 1_024).is_err());
        assert!(validate_signing_metadata(None, None, None, None, Some(&"n".repeat(4_097)), 0).is_err());

        let oversized = FileInput::Bytes {
            data: "A".repeat((MAX_ENVELOPE_BYTES.div_ceil(3) * 4) as usize + 1),
            name: None,
            mime_type: None,
        };
        assert!(validate_file_input_metadata(&oversized, "existingEnvelope").is_err());

        let malformed_base64 = FileInput::Bytes {
            data: "not-base64!?".into(),
            name: None,
            mime_type: None,
        };
        assert!(malformed_base64.load_with(LoadLimits {
            max_bytes: MAX_ENVELOPE_BYTES,
            allow_empty: false,
        }).is_err());
    }

    #[test]
    fn rejects_overlong_trusted_verification_targets() {
        assert!(validate_verification_target(&VerifyTargetDto {
            address: Some("x".repeat(4_097)),
            ..Default::default()
        }).is_err());
    }
}
