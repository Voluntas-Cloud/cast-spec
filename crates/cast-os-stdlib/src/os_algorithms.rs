//! OS algorithms and mechanisms.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod access_check_algorithm;
pub mod block_allocation_algorithm;
pub mod buddy_allocation_algorithm;
pub mod capability_check_algorithm;
pub mod context_switch_algorithm;
pub mod copy_on_write_fault_handling;
pub mod deadlock_detection_algorithm;
pub mod epoch_based_reclamation;
pub mod garbage_collection_for_kernel_objects;
pub mod hazard_pointer_reclamation;
pub mod interrupt_routing_algorithm;
pub mod io_scheduling_algorithm;
pub mod journaling_commit_algorithm;
pub mod load_balancing_algorithm;
pub mod name_cache_lookup;
pub mod packet_classification_algorithm;
pub mod page_replacement_algorithm;
pub mod path_resolution_algorithm;
pub mod priority_inheritance_algorithm;
pub mod rcu_grace_period_algorithm;
pub mod reference_counting;
pub mod scheduler_selection_algorithm;
pub mod slab_allocation_algorithm;
pub mod tcp_congestion_algorithm;

cast::concept! {
    name: "os_algorithms",
    summary: "Umbrella for the os_algorithms stdlib category. OS \
              algorithms and mechanisms.",
    anchors: [
        crate::os_algorithms::access_check_algorithm,
        crate::os_algorithms::block_allocation_algorithm,
        crate::os_algorithms::buddy_allocation_algorithm,
        crate::os_algorithms::capability_check_algorithm,
        crate::os_algorithms::context_switch_algorithm,
        crate::os_algorithms::copy_on_write_fault_handling,
        crate::os_algorithms::deadlock_detection_algorithm,
        crate::os_algorithms::epoch_based_reclamation,
        crate::os_algorithms::garbage_collection_for_kernel_objects,
        crate::os_algorithms::hazard_pointer_reclamation,
        crate::os_algorithms::interrupt_routing_algorithm,
        crate::os_algorithms::io_scheduling_algorithm,
        crate::os_algorithms::journaling_commit_algorithm,
        crate::os_algorithms::load_balancing_algorithm,
        crate::os_algorithms::name_cache_lookup,
        crate::os_algorithms::packet_classification_algorithm,
        crate::os_algorithms::page_replacement_algorithm,
        crate::os_algorithms::path_resolution_algorithm,
        crate::os_algorithms::priority_inheritance_algorithm,
        crate::os_algorithms::rcu_grace_period_algorithm,
        crate::os_algorithms::reference_counting,
        crate::os_algorithms::scheduler_selection_algorithm,
        crate::os_algorithms::slab_allocation_algorithm,
        crate::os_algorithms::tcp_congestion_algorithm,
    ],
    tags: ["cast_os_stdlib", "os_algorithms"],
}

/// Sentinel for the os_algorithms stdlib group.
pub struct OsAlgorithmsGroup;
