# Publishing to crates.io

This package is prepared as `tauri-plugin-majik-signature-client` version `1.2.0` and depends on the published `majik-signature-client = "1.2.0"` crate.

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

Before publishing, confirm that `tauri-plugin-majik-signature-client` version `1.2.0` has not already been published. Crates.io crate versions are immutable; if `1.2.0` is already taken, increment the plugin's version and regenerate the contract's version field before publishing.

A crates.io API token must be configured with `cargo login` before the final `cargo publish` command. This archive does not contain credentials and does not publish anything by itself.

The local source-generation consistency check is separate from Cargo packaging:

```bash
python3 gen.py
python3 tests/check_generated.py
```
