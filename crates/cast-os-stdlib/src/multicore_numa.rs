//! Multi-core, NUMA, and hardware topology.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod atomic_operation;
pub mod cache_hierarchy_awareness;
pub mod cache_line_alignment;
pub mod cpu_topology_model;
pub mod false_sharing_failure_mode;
pub mod lock_free_structure;
pub mod lock_granularity;
pub mod memory_barrier;
pub mod mutex_lock;
pub mod numa_balancing;
pub mod numa_node_model;
pub mod per_cpu_data_structure;
pub mod per_cpu_run_queue;
pub mod rcu_read_copy_update;
pub mod rwlock;
pub mod seqlock;
pub mod simultaneous_multithreading_awareness;
pub mod smp_kernel;
pub mod spinlock;
pub mod wait_free_structure;

cast::concept! {
    name: "multicore_numa",
    summary: "Umbrella for the multicore_numa stdlib category. Multi-core, \
              NUMA, and hardware topology.",
    anchors: [
        crate::multicore_numa::atomic_operation,
        crate::multicore_numa::cache_hierarchy_awareness,
        crate::multicore_numa::cache_line_alignment,
        crate::multicore_numa::cpu_topology_model,
        crate::multicore_numa::false_sharing_failure_mode,
        crate::multicore_numa::lock_free_structure,
        crate::multicore_numa::lock_granularity,
        crate::multicore_numa::memory_barrier,
        crate::multicore_numa::mutex_lock,
        crate::multicore_numa::numa_balancing,
        crate::multicore_numa::numa_node_model,
        crate::multicore_numa::per_cpu_data_structure,
        crate::multicore_numa::per_cpu_run_queue,
        crate::multicore_numa::rcu_read_copy_update,
        crate::multicore_numa::rwlock,
        crate::multicore_numa::seqlock,
        crate::multicore_numa::simultaneous_multithreading_awareness,
        crate::multicore_numa::smp_kernel,
        crate::multicore_numa::spinlock,
        crate::multicore_numa::wait_free_structure,
    ],
    tags: ["cast_os_stdlib", "multicore_numa"],
}

/// Sentinel for the multicore_numa stdlib group.
pub struct MulticoreNumaGroup;
