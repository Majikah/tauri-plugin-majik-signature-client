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
const MAX_OVERRIDE_CHUNK_SIZE: usize = 256 * 1024 * 1024;
const MAX_AUTOMATIC_CHUNK_SIZE: usize = 64 * 1024 * 1024;
const MAX_STREAMING_MEMORY_BUDGET: u64 = 256 * MIB;

/// A privacy-preserving snapshot of resources relevant to streaming operations.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemProfile {
    pub total_memory_bytes: u64,
    pub available_memory_bytes: u64,
    pub host_available_memory_bytes: u64,
    /// Whether sysinfo successfully initialized and returned host memory measurements.
    pub memory_probe_succeeded: bool,
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
        // sysinfo 0.40 makes System initialization fallible on unsupported platforms and
        // when the underlying probes cannot be initialized. Do not panic or block streaming:
        // fall back to finite cgroup limits where available, otherwise use the smallest
        // conservative automatic chunk recommendation.
        let (total_memory, host_available, memory_probe_succeeded) = match System::new() {
            Ok(mut system) => {
                system.refresh_memory();
                (system.total_memory(), system.available_memory(), true)
            }
            Err(_) => (0, 0, false),
        };

        // When sysinfo is unavailable, still allow a finite cgroup limit to inform the
        // recommendation rather than treating every Linux container as having zero memory.
        let cgroup_reference_total = if total_memory == 0 {
            u64::MAX
        } else {
            total_memory
        };
        let cgroup = detect_cgroup_memory_limit(cgroup_reference_total);
        let available = match (memory_probe_succeeded, cgroup) {
            (true, Some((limit, usage))) => {
                host_available.min(limit.saturating_sub(usage))
            }
            (true, None) => host_available,
            (false, Some((limit, usage))) => limit.saturating_sub(usage),
            (false, None) => 0,
        };

        SystemProfile {
            total_memory_bytes: total_memory,
            available_memory_bytes: available,
            host_available_memory_bytes: host_available,
            memory_probe_succeeded,
            cgroup_memory_limit_bytes: cgroup.map(|(limit, _)| limit),
            cpu_threads: thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
            architecture: std::env::consts::ARCH.to_string(),
            operating_system: std::env::consts::OS.to_string(),
        }
    }

    pub fn recommend_streaming_config(&self) -> RecommendedStreamingConfig {
        let profile = self.inspect_system();
        let memory_budget = (profile.available_memory_bytes / 8).min(MAX_STREAMING_MEMORY_BUDGET);
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
    // Conservative available-memory tiers. These are starting defaults, not claims about
    // benchmark-optimal throughput. The application leaves headroom for the OS, webview, user
    // workloads and container metadata. Explicit overrides remain fallible and are retried.
    let mib = available_memory / MIB;

    let recommended = if mib >= 8 * 1024 {
        64 * MIB
    } else if mib >= 4 * 1024 {
        32 * MIB
    } else if mib >= 2 * 1024 {
        16 * MIB
    } else if mib >= 1024 {
        8 * MIB
    } else if mib >= 512 {
        4 * MIB
    } else if mib >= 256 {
        2 * MIB
    } else if mib >= 128 {
        MIB
    } else {
        256 * 1024
    };

    recommended.clamp(MIN_CHUNK_SIZE as u64, MAX_AUTOMATIC_CHUNK_SIZE as u64) as usize
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
        assert_eq!(recommended_chunk_size(0), 256 * 1024);
        assert_eq!(recommended_chunk_size(512 * MIB), 4 * 1024 * 1024);
        assert_eq!(recommended_chunk_size(1024 * MIB), 8 * 1024 * 1024);
        assert_eq!(recommended_chunk_size(2 * 1024 * MIB), 16 * 1024 * 1024);
        assert_eq!(recommended_chunk_size(4 * 1024 * MIB), 32 * 1024 * 1024);
        assert_eq!(recommended_chunk_size(8 * 1024 * MIB), 64 * 1024 * 1024);
        assert!(recommended_chunk_size(u64::MAX) <= MAX_AUTOMATIC_CHUNK_SIZE);
        assert_eq!(MAX_OVERRIDE_CHUNK_SIZE, 256 * MIB);
    }
}
