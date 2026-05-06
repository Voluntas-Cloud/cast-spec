//! Filesystem and storage architecture.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod bind_mount;
pub mod block_device_layer;
pub mod buffer_cache;
pub mod character_device_layer;
pub mod copy_on_write_filesystem;
pub mod crash_consistency;
pub mod delayed_allocation;
pub mod dentry_cache;
pub mod device_mapper_layer;
pub mod distributed_filesystem;
pub mod extent_based_allocation;
pub mod filesystem_namespace;
pub mod fsync_semantics;
pub mod full_data_journaling;
pub mod inode_model;
pub mod journaling_filesystem;
pub mod log_structured_filesystem;
pub mod logical_volume_manager;
pub mod loop_device;
pub mod metadata_journaling;
pub mod mount_namespace;
pub mod network_filesystem;
pub mod object_backed_filesystem;
pub mod overlay_filesystem;
pub mod page_cache;
pub mod snapshot_filesystem;
pub mod software_raid;
pub mod storage_stack_layering;
pub mod subvolume_model;
pub mod union_filesystem;
pub mod virtual_filesystem_layer;
pub mod write_through_cache;
pub mod writeback_cache;

cast::concept! {
    name: "filesystem_storage",
    summary: "Umbrella for the filesystem_storage stdlib category. \
              Filesystem and storage architecture.",
    anchors: [
        crate::filesystem_storage::bind_mount,
        crate::filesystem_storage::block_device_layer,
        crate::filesystem_storage::buffer_cache,
        crate::filesystem_storage::character_device_layer,
        crate::filesystem_storage::copy_on_write_filesystem,
        crate::filesystem_storage::crash_consistency,
        crate::filesystem_storage::delayed_allocation,
        crate::filesystem_storage::dentry_cache,
        crate::filesystem_storage::device_mapper_layer,
        crate::filesystem_storage::distributed_filesystem,
        crate::filesystem_storage::extent_based_allocation,
        crate::filesystem_storage::filesystem_namespace,
        crate::filesystem_storage::fsync_semantics,
        crate::filesystem_storage::full_data_journaling,
        crate::filesystem_storage::inode_model,
        crate::filesystem_storage::journaling_filesystem,
        crate::filesystem_storage::log_structured_filesystem,
        crate::filesystem_storage::logical_volume_manager,
        crate::filesystem_storage::loop_device,
        crate::filesystem_storage::metadata_journaling,
        crate::filesystem_storage::mount_namespace,
        crate::filesystem_storage::network_filesystem,
        crate::filesystem_storage::object_backed_filesystem,
        crate::filesystem_storage::overlay_filesystem,
        crate::filesystem_storage::page_cache,
        crate::filesystem_storage::snapshot_filesystem,
        crate::filesystem_storage::software_raid,
        crate::filesystem_storage::storage_stack_layering,
        crate::filesystem_storage::subvolume_model,
        crate::filesystem_storage::union_filesystem,
        crate::filesystem_storage::virtual_filesystem_layer,
        crate::filesystem_storage::write_through_cache,
        crate::filesystem_storage::writeback_cache,
    ],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}

/// Sentinel for the filesystem_storage stdlib group.
pub struct FilesystemStorageGroup;
