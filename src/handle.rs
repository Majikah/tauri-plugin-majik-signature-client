//! `SigHandle`: the `SignatureClient` behind `Arc<Mutex<_>>`, run on the blocking pool
//! (SQLite, Argon2id and file I/O all block). Streaming cancellation is tracked independently
//! from the client mutex so a cancel command can interrupt a running file operation.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use majik_signature_client::ipc::EVENT_CHANNEL;
use majik_signature_client::{SigEvent, SignatureClient, SignatureClientError};
use tauri::{AppHandle, Emitter, Runtime};

#[derive(Clone)]
pub struct SigHandle {
    inner: Arc<Mutex<SignatureClient>>,
    streams: Arc<Mutex<HashMap<String, Arc<AtomicBool>>>>,
}

impl SigHandle {
    pub fn new(client: SignatureClient) -> Self {
        Self {
            inner: Arc::new(Mutex::new(client)),
            streams: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn run<T, F>(&self, f: F) -> Result<T, SignatureClientError>
    where
        T: Send + 'static,
        F: FnOnce(&mut SignatureClient) -> Result<T, SignatureClientError> + Send + 'static,
    {
        let inner = Arc::clone(&self.inner);
        tauri::async_runtime::spawn_blocking(move || {
            let mut guard = inner.lock().map_err(|_| SignatureClientError::Poisoned)?;
            f(&mut guard)
        })
        .await
        .map_err(|e| SignatureClientError::Storage(format!("blocking task failed: {e}")))?
    }

    /// Run one native-path streaming operation and register its cancellation flag before work
    /// enters the client mutex. Operation IDs are unique while active and bounded in length.
    pub async fn run_stream<T, F>(
        &self,
        operation_id: String,
        f: F,
    ) -> Result<T, SignatureClientError>
    where
        T: Send + 'static,
        F: FnOnce(&mut SignatureClient, Arc<AtomicBool>) -> Result<T, SignatureClientError>
            + Send
            + 'static,
    {
        if operation_id.trim().is_empty() || operation_id.len() > 128 {
            return Err(SignatureClientError::Invalid(
                "Streaming operationId must contain 1–128 bytes".into(),
            ));
        }

        let cancellation = Arc::new(AtomicBool::new(false));
        {
            let mut active = self
                .streams
                .lock()
                .map_err(|_| SignatureClientError::Poisoned)?;
            if active.contains_key(&operation_id) {
                return Err(SignatureClientError::Invalid(format!(
                    "A streaming operation is already active with operationId \"{operation_id}\""
                )));
            }
            active.insert(operation_id.clone(), Arc::clone(&cancellation));
        }

        let inner = Arc::clone(&self.inner);
        let task_result = tauri::async_runtime::spawn_blocking(move || {
            let mut guard = inner.lock().map_err(|_| SignatureClientError::Poisoned)?;
            f(&mut guard, cancellation)
        })
        .await;

        // Always unregister, including when the blocking task panics and its JoinError is set.
        self.streams
            .lock()
            .map_err(|_| SignatureClientError::Poisoned)?
            .remove(&operation_id);

        task_result
            .map_err(|e| SignatureClientError::Storage(format!("streaming task failed: {e}")))?
    }

    /// Request cancellation without taking the client mutex. The active operation checks this
    /// flag between bounded read/write operations; false means the ID is not currently active.
    pub fn cancel_stream(&self, operation_id: &str) -> bool {
        let Ok(active) = self.streams.lock() else {
            return false;
        };
        let Some(cancellation) = active.get(operation_id) else {
            return false;
        };
        cancellation.store(true, Ordering::Release);
        true
    }

    /// Forward every client event (key events and signature events) to the webview.
    pub fn forward_events<R: Runtime>(&self, app: AppHandle<R>) {
        if let Ok(guard) = self.inner.lock() {
            guard.subscribe(Arc::new(move |event: &SigEvent| {
                let _ = app.emit(EVENT_CHANNEL, event);
            }));
        }
    }

    /// Run `f` synchronously from other Rust code (hot-folder, CLI bridge, ...).
    pub fn with_client<T>(&self, f: impl FnOnce(&mut SignatureClient) -> T) -> Option<T> {
        self.inner.lock().ok().map(|mut g| f(&mut g))
    }

    /// Best-effort shutdown wipe of unlocked keys and decrypted stamps.
    pub fn lock_all_blocking(&self) {
        // Signal cancellation before waiting for a possibly busy streaming client operation.
        if let Ok(active) = self.streams.lock() {
            for cancellation in active.values() {
                cancellation.store(true, Ordering::Release);
            }
        }
        if let Ok(mut guard) = self.inner.lock() {
            guard.keys_mut().lock_all_accounts();
            let _ = guard.lock_stamps();
        }
    }
}
