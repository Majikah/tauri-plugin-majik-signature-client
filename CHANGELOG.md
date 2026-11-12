# Changelog

## 1.2.0 - 2026-11-12

- Align the Tauri plugin crate version with `majik-signature-client` 1.2.0.
- Replace the sibling local Cargo path dependency with the published `majik-signature-client = "1.2.0"` dependency.
- Add `verify_file_detached_chain` and `verify_file_revisions_detached` command wrappers for the current Rust IPC API.
- Add the `snapshot` compatibility command while retaining `get_snapshot` for the inherited key-client facade.
- Regenerate command registration, build permission declarations, and the default permission set from one command table.
- Add a machine-readable command contract and a generated-file consistency check.
- Add crates.io package metadata and an explicit package file allowlist.
