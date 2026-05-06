//! Kernel data structures.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod b_tree_index;
pub mod bitmap_allocator;
pub mod buddy_allocator_tree;
pub mod completion_object;
pub mod dentry_tree;
pub mod file_descriptor_table;
pub mod hash_table;
pub mod inode_table;
pub mod lock_dependency_graph;
pub mod mount_tree;
pub mod namespace_graph;
pub mod open_file_table;
pub mod page_cache_tree;
pub mod priority_queue;
pub mod process_table;
pub mod radix_tree_index;
pub mod red_black_tree;
pub mod ring_buffer;
pub mod security_label;
pub mod slab_cache;
pub mod thread_table;
pub mod timer_wheel;
pub mod wait_queue;
pub mod work_queue;
pub mod xarray_index;

cast::concept! {
    name: "kernel_data_structures",
    summary: "Umbrella for the kernel_data_structures stdlib category. \
              Kernel data structures.",
    anchors: [
        crate::kernel_data_structures::b_tree_index,
        crate::kernel_data_structures::bitmap_allocator,
        crate::kernel_data_structures::buddy_allocator_tree,
        crate::kernel_data_structures::completion_object,
        crate::kernel_data_structures::dentry_tree,
        crate::kernel_data_structures::file_descriptor_table,
        crate::kernel_data_structures::hash_table,
        crate::kernel_data_structures::inode_table,
        crate::kernel_data_structures::lock_dependency_graph,
        crate::kernel_data_structures::mount_tree,
        crate::kernel_data_structures::namespace_graph,
        crate::kernel_data_structures::open_file_table,
        crate::kernel_data_structures::page_cache_tree,
        crate::kernel_data_structures::priority_queue,
        crate::kernel_data_structures::process_table,
        crate::kernel_data_structures::radix_tree_index,
        crate::kernel_data_structures::red_black_tree,
        crate::kernel_data_structures::ring_buffer,
        crate::kernel_data_structures::security_label,
        crate::kernel_data_structures::slab_cache,
        crate::kernel_data_structures::thread_table,
        crate::kernel_data_structures::timer_wheel,
        crate::kernel_data_structures::wait_queue,
        crate::kernel_data_structures::work_queue,
        crate::kernel_data_structures::xarray_index,
    ],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}

/// Sentinel for the kernel_data_structures stdlib group.
pub struct KernelDataStructuresGroup;
