//! cast-os-stdlib — a curated library of OS-level architecture
//! concepts.
//!
//! Each `cast::concept!` block in this crate names a recurring
//! OS-level idea (microkernel architecture, completely-fair
//! scheduler, copy-on-write filesystem, namespace isolation, …)
//! with a short summary, and is anchored on a zero-sized sentinel
//! struct so the cast analyzer has a real Rust path to resolve.
//! Downstream projects pull cast-os-stdlib in and reference these
//! concepts through `cast::continues_in!`, inheriting the survey
//! work rather than re-deriving it.
//!
//! Nothing in this crate executes.

pub mod architectural_patterns;
pub mod boot_init;
pub mod configuration;
pub mod design_qualities;
pub mod desktop_session;
pub mod distributed_os;
pub mod driver_model;
pub mod execution_model;
pub mod failure_modes;
pub mod fault_recovery;
pub mod filesystem_storage;
pub mod interrupts_timers;
pub mod io_architecture;
pub mod ipc;
pub mod isolation;
pub mod kernel_architecture;
pub mod kernel_data_structures;
pub mod kernel_families;
pub mod kernel_user_boundary;
pub mod memory_management;
pub mod multicore_numa;
pub mod networking;
pub mod observability;
pub mod os_algorithms;
pub mod os_use_cases;
pub mod power_thermal;
pub mod realtime;
pub mod scheduling;
pub mod security;
pub mod self_adaptive_os;
pub mod service_management;
pub mod update_evolution;
pub mod virtualization;

cast::concept! {
    name: "cast_os_stdlib",
    summary: "Per-crate umbrella for the OS-level architecture concept \
              catalog. Pulls every concept under one of the 33 category \
              modules (kernel_architecture, scheduling, realtime, \
              memory_management, …) underneath this node by \
              longest-prefix-match on the module-level anchors below. \
              Without this umbrella, cast-os-stdlib concepts are orphans \
              in the canonical tree: their anchors strict-prefix-match \
              no other concept's anchor, so `place_zero_anchor` falls \
              back to picking an alphabetically-first sibling and the \
              resulting placement cycle disconnects the whole crate \
              from the workspace root.",
    anchors: [
        crate::architectural_patterns,
        crate::boot_init,
        crate::configuration,
        crate::design_qualities,
        crate::desktop_session,
        crate::distributed_os,
        crate::driver_model,
        crate::execution_model,
        crate::failure_modes,
        crate::fault_recovery,
        crate::filesystem_storage,
        crate::interrupts_timers,
        crate::io_architecture,
        crate::ipc,
        crate::isolation,
        crate::kernel_architecture,
        crate::kernel_data_structures,
        crate::kernel_families,
        crate::kernel_user_boundary,
        crate::memory_management,
        crate::multicore_numa,
        crate::networking,
        crate::observability,
        crate::os_algorithms,
        crate::os_use_cases,
        crate::power_thermal,
        crate::realtime,
        crate::scheduling,
        crate::security,
        crate::self_adaptive_os,
        crate::service_management,
        crate::update_evolution,
        crate::virtualization,
    ],
    tags: ["cast_os_stdlib"],
}
