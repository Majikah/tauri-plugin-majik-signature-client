# Publishing to crates.io

This package is prepared as `tauri-plugin-majik-signature-client` version `1.6.0` and depends on the published `majik-signature-client = "1.6.0"` crate.

Run these commands from the directory containing this crate's `Cargo.toml`:

```bash
# Confirm the package file set and metadata.
cargo package --list

# Check package creation and compile the packaged crate in a clean context.
cargo package

# Validate the crates.io publishing flow without uploading.
cargo publish --dry-run

# Publish after reviewing the dry-run output.
cargo publish
```

Before publishing, confirm that `tauri-plugin-majik-signature-client` version `1.6.0` has not already been published. Crates.io crate versions are immutable; if `1.6.0` is already taken, increment the plugin's version and regenerate the contract's version field before publishing.

A crates.io API token must be configured with `cargo login` before the final `cargo publish` command. This archive does not contain credentials and does not publish anything by itself.

The local source-generation consistency check is separate from Cargo packaging:

```bash
python3 gen.py
python3 tests/check_generated.py
```


## Upstream release gate

The 1.6.0 plugin source currently uses the merged majik-signature-client Git revision through a development [patch.crates-io] entry. Before publishing, verify that majik-signature-client 1.6.0 is available on crates.io and that it includes sign_batch_detached_paths_stream and verify_mjks_map_paths_stream. Cargo omits [patch] from the packaged manifest. Do not publish this plugin before the upstream client release.


## Upstream 1.6.0 dependency release gate

This branch builds against the merged `majik-signature-client` 1.6.0 Git revision through a development `[patch.crates-io]` entry in `Cargo.toml`, because the registry release was not yet available when this PR was prepared. The patch table is omitted from the packaged plugin manifest. Before publishing:

1. Publish `majik-signature-client` 1.6.0 to crates.io.
2. Confirm `cargo info majik-signature-client@1.6.0` resolves from the registry and that the registry package exposes `sign_batch_detached_paths_stream` and `verify_mjks_map_paths_stream`.
3. Remove the development `[patch.crates-io]` block from the plugin's `Cargo.toml`.
4. Run the full plugin CI and `cargo package` / `cargo publish --dry-run` again against the registry dependency.
5. Publish `tauri-plugin-majik-signature-client` 1.6.0 only after those checks pass.

**Do not publish this plugin crate while its required Rust client 1.6.0 has not been published to crates.io.**
