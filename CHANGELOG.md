# Changelog

## 1.5.1 - 2026-11-27

- Add a plugin-managed SystemResourceManager using sysinfo 0.38.4, preserving the Rust 1.92 MSRV.
- Expose system-profile and recommended-streaming-configuration commands.
- Resolve omitted chunk sizes from current available memory and common Linux cgroup memory limits.
- Support explicit 64 KiB–128 MiB overrides and adaptive allocation retries with progressive downgrade.
- Add retry metadata to streaming progress events and keep atomic output cleanup as the retry boundary.
- Depend on majik-signature-client 1.5.1 and regenerate command permissions and contract.

