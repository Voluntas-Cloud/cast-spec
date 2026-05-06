//! Distributed and cluster OS concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod cluster_membership_service;
pub mod cluster_recovery_protocol;
pub mod cluster_scheduler;
pub mod data_locality_scheduling;
pub mod distributed_filesystem_namespace;
pub mod distributed_lock_manager;
pub mod distributed_process_model;
pub mod distributed_resource_accounting;
pub mod distributed_shared_memory;
pub mod fencing_failed_node;
pub mod heartbeat_failure_detection;
pub mod leader_election_service;
pub mod node_capability_advertisement;
pub mod process_migration;
pub mod quorum_membership;
pub mod remote_device_access;
pub mod remote_syscall_model;
pub mod single_system_image;
pub mod split_brain_failure_mode;

cast::concept! {
    name: "distributed_os",
    summary: "Umbrella for the distributed_os stdlib category. Distributed \
              and cluster OS concepts.",
    anchors: [
        crate::distributed_os::cluster_membership_service,
        crate::distributed_os::cluster_recovery_protocol,
        crate::distributed_os::cluster_scheduler,
        crate::distributed_os::data_locality_scheduling,
        crate::distributed_os::distributed_filesystem_namespace,
        crate::distributed_os::distributed_lock_manager,
        crate::distributed_os::distributed_process_model,
        crate::distributed_os::distributed_resource_accounting,
        crate::distributed_os::distributed_shared_memory,
        crate::distributed_os::fencing_failed_node,
        crate::distributed_os::heartbeat_failure_detection,
        crate::distributed_os::leader_election_service,
        crate::distributed_os::node_capability_advertisement,
        crate::distributed_os::process_migration,
        crate::distributed_os::quorum_membership,
        crate::distributed_os::remote_device_access,
        crate::distributed_os::remote_syscall_model,
        crate::distributed_os::single_system_image,
        crate::distributed_os::split_brain_failure_mode,
    ],
    tags: ["cast_os_stdlib", "distributed_os"],
}

/// Sentinel for the distributed_os stdlib group.
pub struct DistributedOsGroup;
