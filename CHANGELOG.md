# Changelog

## 1.5.1 - 2026-11-27

- Add a plugin-managed `SystemResourceManager` using `sysinfo 0.40.4`.
- Handle fallible system initialization without panicking; use finite Linux cgroup limits when host memory probing is unavailable.
- Raise the plugin MSRV to Rust 1.95 to match `sysinfo 0.40.4`.
- Expose system-profile and recommended-streaming-configuration commands.
- Resolve omitted chunk sizes from current available memory.
- Support explicit 64 KiB–256 MiB overrides; automatic recommendations remain capped at 64 MiB.
- Retry allocation failures using the system recommendation and progressively smaller chunk sizes.
- Add retry metadata to streaming progress events and regenerate command permissions and contract.
- Fix the recommendation clamp's `u64`/`usize` type mismatch.
