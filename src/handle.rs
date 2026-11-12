//! `SigHandle`: the `SignatureClient` behind `Arc<Mutex<_>>`, run on the blocking pool (SQLite,
//! Argon2id and file I/O all block).

use std::sync::{Arc, Mutex};

use majik_signature_client::ipc::EVENT_CHANNEL;
use majik_signature_client::{SigEvent, SignatureClient, SignatureClientError};
use tauri::{AppHandle, Emitter, Runtime};

#[derive(Clone)]
pub struct SigHandle(Arc<Mutex<SignatureClient>>);

impl SigHandle {
    pub fn new(client: SignatureClient) -> Self {
        Self(Arc::new(Mutex::new(client)))
    }

    pub async fn run<T, F>(&self, f: F) -> Result<T, SignatureClientError>
    where
        T: Send + 'static,
        F: FnOnce(&mut SignatureClient) -> Result<T, SignatureClientError> + Send + 'static,
    {
        let inner = Arc::clone(&self.0);
        tauri::async_runtime::spawn_blocking(move || {
            let mut guard = inner.lock().map_err(|_| SignatureClientError::Poisoned)?;
            f(&mut guard)
        })
        .await
        .map_err(|e| SignatureClientError::Storage(format!("blocking task failed: {e}")))?
    }

    /// Forward every client event (key events and signature events) to the webview.
    pub fn forward_events<R: Runtime>(&self, app: AppHandle<R>) {
        if let Ok(guard) = self.0.lock() {
            guard.subscribe(Arc::new(move |event: &SigEvent| {
                let _ = app.emit(EVENT_CHANNEL, event);
            }));
        }
    }

    /// Run `f` synchronously from other Rust code (hot-folder, CLI bridge, ...).
    pub fn with_client<T>(&self, f: impl FnOnce(&mut SignatureClient) -> T) -> Option<T> {
        self.0.lock().ok().map(|mut g| f(&mut g))
    }

    /// Best-effort shutdown wipe of unlocked keys and decrypted stamps.
    pub fn lock_all_blocking(&self) {
        if let Ok(mut guard) = self.0.lock() {
            guard.keys_mut().lock_all_accounts();
            let _ = guard.lock_stamps();
        }
    }
}
