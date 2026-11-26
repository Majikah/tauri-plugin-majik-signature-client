# Changelog

## 1.5.0 - 2026-11-27

- Upgrade Tauri to 2.12.7, `tauri-plugin` to 2.7.4, and rusqlite to 0.43.0.
- Depend on `majik-signature-client` 1.5.0 and raise the minimum supported Rust version to 1.92.
- Expose path-backed embedded and detached streaming signing and verification, including native-path revision signing.
- Emit bounded-memory streaming progress events and add operation-ID based cancellation independent of the client mutex.
- Regenerate the command registration, default permissions, and machine-readable contract for the expanded command surface.

## 1.3.3 - 2026-11-25

- Expose the session-bound detached signing operation for SignRoom through the Tauri plugin command surface.
- Depend on `majik-signature-client` 1.3.3, which provides the native Rust IPC operation.


## 1.2.0 - 2026-11-12

- Align the Tauri plugin crate version with `majik-signature-client` 1.2.0.
- Replace the sibling local Cargo path dependency with the published `majik-signature-client = "1.2.0"` dependency.
- Add `verify_file_detached_chain` and `verify_file_revisions_detached` command wrappers for the current Rust IPC API.
- Add the `snapshot` compatibility command while retaining `get_snapshot` for the inherited key-client facade.
- Regenerate command registration, build permission declarations, and the default permission set from one command table.
- Add a machine-readable command contract and a generated-file consistency check.
- Add crates.io package metadata and an explicit package file allowlist.
