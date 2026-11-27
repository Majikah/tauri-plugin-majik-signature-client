//! Conservative system-resource profiling and streaming recommendations.
//!
//! This module intentionally lives in the Tauri plugin: hardware inspection is an application
//! concern and the lower-level signature crates remain independent of Tauri and sysinfo.

use std::fs;
use std::thread;

use serde::Serialize;
use sysinfo::System;

const MIB: u64 = 1024 * 1024;
const MIN_CHUNK_SIZE: usize = 64 * 1024;
const MAX_OVERRIDE_CHUNK_SIZE: usize = 128 * 1024 * 1024;
const MAX_AUTOMATIC_CHUNK_SIZE: usize = 64 * 1024 * 1024;
const MAX_STREAMING_MEMORY_BUDGET: u64 = 256 * MIB;

/// A privacy-preserving snapshot of resources relevant to streaming operations.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemProfile {
    pub total_memory_bytes: u64,
    pub available_memory_bytes: u64,
    pub host_available_memory_bytes: u64,
    pub cgroup_memory_limit_bytes: Option<u64>,
    pub cpu_threads: usize,
    pub architecture: String,
    pub operating_system: String,
}

/// The plugin's current recommended defaults for a single streaming operation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedStreamingConfig {
    pub chunk_size: usize,
    pub memory_budget_bytes: u64,
    pub maximum_override_chunk_size: usize,
    pub automatic_max_chunk_size: usize,
    pub profile: &'static str,
}

/// Stateful Tauri-managed facade. The current implementation refreshes memory on demand so
/// recommendations reflect current pressure rather than a stale startup snapshot.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemResourceManager;

impl SystemResourceManager {
    pub fn inspect_system(&self) -> SystemProfile {
        let mut system = System::new();
        system.refresh_memory();

        let total_memory = system.total_memory();
        let host_available = system.available_memory();
        let cgroup = detect_cgroup_memory_limit(total_memory);
        let available = cgroup
            .map(|(limit, usage)| host_available.min(limit.saturating_sub(usage)))
            .unwrap_or(host_available);

        SystemProfile {
            total_memory_bytes: total_memory,
            available_memory_bytes: available,
            host_available_memory_bytes: host_available,
            cgroup_memory_limit_bytes: cgroup.map(|(limit, _)| limit),
            cpu_threads: thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
            architecture: std::env::consts::ARCH.to_string(),
            operating_system: std::env::consts::OS.to_string(),
        }
    }

    pub fn recommend_streaming_config(&self) -> RecommendedStreamingConfig {
        let profile = self.inspect_system();
        let memory_budget = (profile.available_memory_bytes / 8)
            .min(MAX_STREAMING_MEMORY_BUDGET);
        let chunk_size = recommended_chunk_size(profile.available_memory_bytes);

        RecommendedStreamingConfig {
            chunk_size,
            memory_budget_bytes: memory_budget,
            maximum_override_chunk_size: MAX_OVERRIDE_CHUNK_SIZE,
            automatic_max_chunk_size: MAX_AUTOMATIC_CHUNK_SIZE,
            profile: if chunk_size >= 32 * 1024 * 1024 {
                "highPerformance"
            } else if chunk_size >= 8 * 1024 * 1024 {
                "balanced"
            } else {
                "lowMemory"
            },
        }
    }
}

fn recommended_chunk_size(available_memory: u64) -> usize {
    // Reserve at least seven eighths of currently available RAM for the OS, webview, application
    // state and other workloads. Budget for two in-flight buffers (copy + hash), and cap automatic
    // recommendations at 64 MiB. 128 MiB remains an explicit, fallible override.
    let budget = (available_memory / 8).min(MAX_STREAMING_MEMORY_BUDGET);
    let ceiling = (budget / 2).min(MAX_AUTOMATIC_CHUNK_SIZE as u64);

    [
        64 * MIB,
        32 * MIB,
        16 * MIB,
        8 * MIB,
        4 * MIB,
        2 * MIB,
        MIB,
        512 * 1024,
        256 * 1024,
        128 * 1024,
        MIN_CHUNK_SIZE as u64,
    ]
    .into_iter()
    .find(|candidate| *candidate <= ceiling)
    .unwrap_or(MIN_CHUNK_SIZE as u64) as usize
}

fn detect_cgroup_memory_limit(total_memory: u64) -> Option<(u64, u64)> {
    // cgroup v2. "max" means that this cgroup does not impose a memory ceiling.
    if let (Ok(limit), Ok(usage)) = (
        fs::read_to_string("/sys/fs/cgroup/memory.max"),
        fs::read_to_string("/sys/fs/cgroup/memory.current"),
    ) {
        if let (Ok(limit), Ok(usage)) = (limit.trim().parse::<u64>(), usage.trim().parse::<u64>()) {
            if limit > 0 && limit < total_memory.saturating_mul(2) {
                return Some((limit, usage));
            }
        }
    }

    // cgroup v1. Large sentinel values are treated as "unlimited".
    if let (Ok(limit), Ok(usage)) = (
        fs::read_to_string("/sys/fs/cgroup/memory/memory.limit_in_bytes"),
        fs::read_to_string("/sys/fs/cgroup/memory/memory.usage_in_bytes"),
    ) {
        if let (Ok(limit), Ok(usage)) = (limit.trim().parse::<u64>(), usage.trim().parse::<u64>()) {
            if limit > 0 && limit < total_memory.saturating_mul(2) {
                return Some((limit, usage));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommendations_are_bounded_and_scale_with_available_memory() {
        assert_eq!(recommended_chunk_size(0), MIN_CHUNK_SIZE);
        assert_eq!(recommended_chunk_size(512 * MIB), 32 * 1024 * 1024);
        assert_eq!(recommended_chunk_size(4 * 1024 * MIB), 64 * 1024 * 1024);
        assert!(recommended_chunk_size(u64::MAX) <= MAX_AUTOMATIC_CHUNK_SIZE);
    }
}
