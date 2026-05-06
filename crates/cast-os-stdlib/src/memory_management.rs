//! Memory management concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod address_space;
pub mod anonymous_memory;
pub mod buddy_allocator;
pub mod clock_page_replacement;
pub mod copy_on_write_memory;
pub mod demand_paging;
pub mod dma_buffer_mapping;
pub mod guard_page;
pub mod huge_pages;
pub mod iommu_memory_protection;
pub mod kernel_address_space_layout_randomization;
pub mod kmalloc_allocator;
pub mod least_recently_used_page_policy;
pub mod major_page_fault;
pub mod memory_ballooning;
pub mod memory_compaction;
pub mod memory_mapped_file;
pub mod memory_page;
pub mod memory_pressure_detection;
pub mod minor_page_fault;
pub mod numa_memory_policy;
pub mod oom_killer;
pub mod page_fault;
pub mod page_frame;
pub mod page_replacement_algorithm;
pub mod page_table;
pub mod physical_memory;
pub mod shared_memory_segment;
pub mod slab_allocator;
pub mod swap_space;
pub mod transparent_huge_pages;
pub mod user_address_space_layout_randomization;
pub mod virtual_memory;
pub mod vmalloc_allocator;
pub mod working_set_model;
pub mod zero_copy_buffering;

cast::concept! {
    name: "memory_management",
    summary: "Umbrella for the memory_management stdlib category. Memory \
              management concepts.",
    anchors: [
        crate::memory_management::address_space,
        crate::memory_management::anonymous_memory,
        crate::memory_management::buddy_allocator,
        crate::memory_management::clock_page_replacement,
        crate::memory_management::copy_on_write_memory,
        crate::memory_management::demand_paging,
        crate::memory_management::dma_buffer_mapping,
        crate::memory_management::guard_page,
        crate::memory_management::huge_pages,
        crate::memory_management::iommu_memory_protection,
        crate::memory_management::kernel_address_space_layout_randomization,
        crate::memory_management::kmalloc_allocator,
        crate::memory_management::least_recently_used_page_policy,
        crate::memory_management::major_page_fault,
        crate::memory_management::memory_ballooning,
        crate::memory_management::memory_compaction,
        crate::memory_management::memory_mapped_file,
        crate::memory_management::memory_page,
        crate::memory_management::memory_pressure_detection,
        crate::memory_management::minor_page_fault,
        crate::memory_management::numa_memory_policy,
        crate::memory_management::oom_killer,
        crate::memory_management::page_fault,
        crate::memory_management::page_frame,
        crate::memory_management::page_replacement_algorithm,
        crate::memory_management::page_table,
        crate::memory_management::physical_memory,
        crate::memory_management::shared_memory_segment,
        crate::memory_management::slab_allocator,
        crate::memory_management::swap_space,
        crate::memory_management::transparent_huge_pages,
        crate::memory_management::user_address_space_layout_randomization,
        crate::memory_management::virtual_memory,
        crate::memory_management::vmalloc_allocator,
        crate::memory_management::working_set_model,
        crate::memory_management::zero_copy_buffering,
    ],
    tags: ["cast_os_stdlib", "memory_management"],
}

/// Sentinel for the memory_management stdlib group.
pub struct MemoryManagementGroup;
