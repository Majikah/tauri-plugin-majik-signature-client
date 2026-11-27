# tauri-plugin-majik-signature-client

**Majikah's Tauri v2 plugin for `majik-signature-client`.** It exposes the Rust signature client to a Tauri webview through typed Tauri commands, a primary event channel, and a dedicated streaming progress channel. Key management, signing, verification, contacts, groups, stamps, logs, preferences, and backup/restore remain in Rust.

- **Crate version:** `1.5.1`
- **Rust client:** `majik-signature-client = 1.5.1`
- **Tauri:** `2.12.7`
- **Commands:** 149 (including key-client commands, compatibility aliases, and native streaming operations)
- **Event channel:** `majik-signature-client:event`
- **Streaming progress channel:** `majik-signature-client:stream-progress`
- **License:** Apache-2.0

## Add the dependency

```toml
[dependencies]
tauri-plugin-majik-signature-client = "1.5.0"
```

The plugin depends on the published `majik-signature-client` crate. It does not require a sibling checkout or a local `path` dependency.

## Register the plugin with a shared SQLite connection

The signature client and the host application must use the same SQLite connection when they share the app database. The `SharedConnection` type exported by `majik-signature-client` is an `Arc<Mutex<rusqlite::Connection>>`.

```rust
use std::sync::{Arc, Mutex};
use majik_signature_client::SharedConnection;
use tauri::Manager;

// In your Tauri builder's .setup(|app| { ... }) callback, after opening
// and configuring the SQLCipher connection:
let connection: SharedConnection = Arc::new(Mutex::new(open_sqlcipher_connection(app)?));

// Keep using this same connection for your app-owned database commands.
app.manage(AppDatabaseState(Arc::clone(&connection)));

// Register the signature plugin.
app.handle().plugin(
    tauri_plugin_majik_signature_client::init(Arc::clone(&connection))
)?;
```

`open_sqlcipher_connection` and `AppDatabaseState` above represent the host application's existing database setup and state types. Apply your SQLCipher key before any query is executed, then pass the initialized connection to the plugin.

The plugin constructs `SignatureClient` with this shared connection, manages a single `SigHandle`, and forwards the client's key and signature events to the webview. The client performs its storage hydration when the frontend opens the session.

## Enable the plugin capability

Add the plugin's default permission set to the capability used by the target webview:

```json
{
  "permissions": [
    "core:default",
    "majik-signature-client:default"
  ]
}
```

The crate ships `permissions/default.toml` and generates per-command allow/deny permissions through `build.rs`. The default permission set enables the plugin's full command API. Applications with a stricter permission model can grant only the individual `majik-signature-client:allow-*` permissions they need instead.

## Use the Rust handle from other Tauri subsystems

The plugin publicly exports `SigHandle` for host-side Rust code such as hot-folder processing, CLI bridges, and application services:

```rust
use tauri::Manager;
use tauri_plugin_majik_signature_client::SigHandle;

let handle = app.state::<SigHandle>();

// Synchronous access is intended for short host-side operations.
let result = handle.with_client(|client| {
    client.is_signing_session_active()
});
```

For operations that may block, use the async command API or move work to an appropriate blocking task. `SigHandle::run` executes client operations on Tauri's blocking pool.

## Frontend integration

Use the companion TypeScript package `@majikah/majik-signature-client-tauri`. It extends the key-client facade and invokes this plugin by its registered name, `majik-signature-client`.

```ts
import { MajikSignatureClientTauri } from "@majikah/majik-signature-client-tauri";

const client = await MajikSignatureClientTauri.open({
  onUnlockRequested: async (accountId) => requestPassphrase(accountId),
});

const accounts = client.listOwnAccounts(); // synchronous read-model
await client.createAccount({ mnemonic, passphrase, label: "Main" });
await client.signFile({ input: { path: "/path/to/document.pdf" } });
```

The frontend facade keeps a small synchronous read-model for account state. Commands and mutations execute in Rust. Existing `FileInput` commands accept `{ path }` or small inline base64 payloads, but those legacy operations still materialize the file in memory (with the client's 1 GiB default limit). Use the native-path streaming commands below for large media and documents; they never transfer source-file bytes through the webview.

## Path-based streaming file operations

Use the streaming commands when files may be large (including multi-gigabyte MOV/MP4 files). These commands accept native paths directly; they do not call the legacy `FileInput::load()` path, return base64 file contents, or copy the entire source into the encrypted history log.

| Command | Purpose |
|---|---|
| `sign_file_embedded_stream` | Stream-sign a file and atomically publish the signed output |
| `sign_file_detached_stream` | Stream-sign a file and atomically publish a small `.mjksig` sidecar |
| `verify_file_embedded_stream` | Verify an embedded signature from a native path |
| `verify_file_detached_stream` | Verify a native file against a native `.mjksig` sidecar |
| `sign_file_embedded_stream_with_prior` | Verify a previous signed file path and append an embedded revision |
| `sign_file_detached_stream_with_prior` | Verify a prior file path and sidecar, then append a detached revision |
| `cancel_stream` | Cancel an active streaming operation by its operation ID |

The plugin profiles available RAM (including common cgroup v1/v2 limits) and recommends a bounded automatic chunk size. Explicit TypeScript `chunkSize` overrides are honored within 64 KiB–128 MiB. If an allocation returns the stable `memoryAllocation` error, the plugin retries with the lower system recommendation and then progressively smaller sizes unless `strictChunkSize` is enabled. Progress events identify the attempt and effective chunk size so the frontend can reset progress when a retry begins.

The `get_system_profile` and `get_recommended_streaming_config` commands expose the same profile and recommendation for diagnostics; streaming commands resolve the recommendation internally at operation start.

Each streaming request carries a unique `operationId`, native `inputPath` and `outputPath` values, and optional `streaming` settings: `chunkSize` (64 KiB–128 MiB; omitted defaults to a system recommendation; 1 MiB is the Rust client fallback) and `maxBytes` (no default total-file-size cap). Signing accepts an `overwrite` flag; it defaults to `false`. Revision methods require `priorFilePath`; the legacy in-memory `priorFile` / `priorSignedFile` option is rejected for these methods.

Progress is emitted on the dedicated `majik-signature-client:stream-progress` event channel as `{ type: "stream-progress", payload: { operationId, bytesProcessed, totalBytes } }`. This separate channel avoids refreshing the signature read-model for high-frequency progress notifications. Invoke `cancel_stream` with the same `operationId` to request cancellation. Cancellation is checked between bounded read/write steps and can be requested without acquiring the signature client's mutex. The returned signing result contains signature metadata and output path/size only, never the source file bytes.

Streaming supports PDF, MP4/MOV, PNG/JPEG, MP3/FLAC, standard RIFF WAV, Office/ODF ZIP containers, Matroska/WebM, and the client's universal-trailer fallback. MP4/MOV embedded rewriting is intentionally restricted to layouts where a final `moov` follows `mdat`; unsafe layouts should use detached signing. Office archives are staged on disk with bounded metadata (32 MiB central directory and 8 MiB embedded signature entry) and the core 512 MiB total uncompressed-content limit. Text/markup, MSIX/AppX, generic unrecognized ZIP, and RF64 WAV are not supported by the streaming API. Refer to the Rust client crate documentation for the format-specific constraints.

For example, listen for the progress event before invoking a command:

```ts
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const operationId = crypto.randomUUID();
const unlisten = await listen("majik-signature-client:event", ({ payload }) => {
  if (payload.type === "stream-progress" && payload.payload.operationId === operationId) {
    console.log(payload.payload.bytesProcessed, payload.payload.totalBytes);
  }
});

try {
  await invoke("plugin:majik-signature-client|sign_file_embedded_stream", {
    args: {
      operationId,
      inputPath: "/path/to/large-video.mov",
      outputPath: "/path/to/large-video.signed.mov",
      options: { mimeType: "video/quicktime" },
      streaming: { chunkSize: 1048576 },
      overwrite: false
    }
  });
} finally {
  unlisten();
}
```

## Command surface

The generated wire contract is maintained in [`contract/commands.json`](contract/commands.json). It records every command name, its Tauri argument list, return type, plugin version, event channel, and event names.

The 149 commands cover:

| Area | Operations |
|---|---|
| Session and accounts | Open session, snapshot, hydrate, mnemonic generation, account creation/import/export, account selection, lock/unlock, passphrase changes, capability checks, keystore migration and restore |
| State and preferences | Client state get/set/remove/list; get/set/reset user preferences |
| Signing sessions | Begin, end, force-end and inspect signing sessions |
| Contacts and groups | Contact lookup/CRUD/import/export, Majikah status, block/unblock, group CRUD and membership/favorites operations |
| Stamps | Create, list, hydrate, lock, inspect, decrypt, replace, rename and remove stamps |
| Signing and TSA | Text signing, file and detached signing, resigning, batch signing, two-step TSA signing and cancellation |
| Verification | Text, embedded, detached, all-signature, chain, revision, signature extraction and signature stripping |
| Sealing and anchoring | Seal, verify seal, inspect seal, check anchor eligibility and embed a confirmed chain anchor |
| Signature maps | Describe, sign, co-sign, verify maps and verify signing order |
| Backups | Back up, inspect, read, restore and selectively restore contacts, stamps and application data |
| Logs | List, clear, restart and hydrate history/activity logs |

The command list includes both `get_snapshot` (the inherited key-client facade command) and `snapshot` (the existing signature-facade compatibility method). The detached-chain and detached-revision verification commands are included and call the corresponding Rust IPC operations.

Not every public Rust method is a separate IPC command. Internal accessors, subscription plumbing, closure-based helpers, and APIs that return non-serializable Rust references remain Rust-only by design. The command set exposes the serializable application operations defined by `majik_signature_client::ipc::ops`, together with the inherited key-client IPC operations required by the frontend facade.

## Events

The plugin forwards events on `majik-signature-client:event`. Each event uses the `{ "type": "...", "payload": ... }` envelope expected by the TypeScript facade.

The primary event channel includes the key-client lifecycle events (account creation/removal/update, active-account changes, lock/unlock, errors and backup restore) and signature-client state events:

- `sign`
- `verify`
- `new-stamp`
- `removed-stamp`
- `new-contact`
- `new-contact-group`
- `removed-contact`
- `removed-contact-group`
- `contact-group-change`
- `history-log`
- `activity-log`

High-volume streaming progress notifications are emitted separately on `majik-signature-client:stream-progress` as `{ type: "stream-progress", payload: { operationId, bytesProcessed, totalBytes } }`. This keeps progress updates out of the inherited snapshot-refresh event pipeline.

## Locking and shutdown

`lock_account` and `lock_all_accounts` also ask the signature client to lock decrypted stamp material. On Tauri application exit, the plugin best-effort locks all accounts and stamps again. If the application hides to the system tray without exiting, the host application should explicitly call `lockAllAccounts()` when its security policy requires locking on minimize.

## Regenerate and verify the command surface

`gen.py` is the single source of truth for command definitions. It generates:

- `src/commands.rs`
- `src/lib.rs` command registration
- The hand-written native-path streaming handlers in `src/stream_commands.rs` are checked against the contract by `tests/check_generated.py`
- `build.rs` permission generation manifest
- `permissions/default.toml`
- `contract/commands.json`

After changing the command table, run:

```bash
python3 gen.py
python3 tests/check_generated.py
```

The contract check verifies both generated command groups, their Tauri handler registrations, the build manifest, and the default permission list against the same unique command set.

## Compatibility

- `tauri-plugin-majik-signature-client` `1.5.0`
- `majik-signature-client` `1.5.0`
- Tauri `2.12.7`
- Rust `1.92` or newer

## License

Apache-2.0. See [LICENSE](LICENSE).
