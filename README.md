# tauri-plugin-majik-signature-client

**Majikah's Tauri v2 plugin for `majik-signature-client`.** It exposes the Rust signature client to a Tauri webview through typed Tauri commands and a single event channel. Key management, signing, verification, contacts, groups, stamps, logs, preferences, and backup/restore remain in Rust.

- **Crate version:** `1.2.0`
- **Rust client:** `majik-signature-client = 1.2.0`
- **Tauri:** v2
- **Commands:** 137 (including the key-client command surface and compatibility aliases)
- **Event channel:** `majik-signature-client:event`
- **License:** Apache-2.0

## Add the dependency

```toml
[dependencies]
tauri-plugin-majik-signature-client = "1.2.0"
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

The frontend facade keeps a small synchronous read-model for account state. Commands and mutations execute in Rust. File inputs should normally use `{ path }`, so Rust can load and validate the file without transferring large file contents through the webview. Inline base64 input remains available for small payloads; output data is returned inline when no output path is specified.

## Command surface

The generated wire contract is maintained in [`contract/commands.json`](contract/commands.json). It records every command name, its Tauri argument list, return type, plugin version, event channel, and event names.

The 137 commands cover:

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

The stream includes the key-client lifecycle events (account creation/removal/update, active-account changes, lock/unlock, errors and backup restore) and signature-client events:

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

## Locking and shutdown

`lock_account` and `lock_all_accounts` also ask the signature client to lock decrypted stamp material. On Tauri application exit, the plugin best-effort locks all accounts and stamps again. If the application hides to the system tray without exiting, the host application should explicitly call `lockAllAccounts()` when its security policy requires locking on minimize.

## Regenerate and verify the command surface

`gen.py` is the single source of truth for command definitions. It generates:

- `src/commands.rs`
- `src/lib.rs` command registration
- `build.rs` permission generation manifest
- `permissions/default.toml`
- `contract/commands.json`

After changing the command table, run:

```bash
python3 gen.py
python3 tests/check_generated.py
```

The contract check verifies that the generated handler, command functions, build manifest, and default permission list all contain the same unique command set.

## Compatibility

- `tauri-plugin-majik-signature-client` `1.2.0`
- `majik-signature-client` `1.2.0`
- Tauri `2.x`
- Rust `1.80` or newer

## License

Apache-2.0. See [LICENSE](LICENSE).
