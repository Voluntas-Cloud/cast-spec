//! OS failure modes.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod abi_breakage;
pub mod boot_failure;
pub mod container_escape;
pub mod device_firmware_hang;
pub mod driver_crash;
pub mod filesystem_corruption;
pub mod interrupt_storm;
pub mod io_starvation;
pub mod kernel_deadlock;
pub mod livelock;
pub mod lock_contention_collapse;
pub mod memory_leak_kernel;
pub mod null_pointer_dereference_kernel;
pub mod priority_inversion_failure;
pub mod privilege_escalation;
pub mod race_condition_kernel;
pub mod scheduler_starvation;
pub mod security_policy_bypass;
pub mod update_brick;
pub mod use_after_free_kernel;
pub mod vm_escape;

cast::concept! {
    name: "failure_modes",
    summary: "Umbrella for the failure_modes stdlib category. OS failure \
              modes.",
    anchors: [
        crate::failure_modes::abi_breakage,
        crate::failure_modes::boot_failure,
        crate::failure_modes::container_escape,
        crate::failure_modes::device_firmware_hang,
        crate::failure_modes::driver_crash,
        crate::failure_modes::filesystem_corruption,
        crate::failure_modes::interrupt_storm,
        crate::failure_modes::io_starvation,
        crate::failure_modes::kernel_deadlock,
        crate::failure_modes::livelock,
        crate::failure_modes::lock_contention_collapse,
        crate::failure_modes::memory_leak_kernel,
        crate::failure_modes::null_pointer_dereference_kernel,
        crate::failure_modes::priority_inversion_failure,
        crate::failure_modes::privilege_escalation,
        crate::failure_modes::race_condition_kernel,
        crate::failure_modes::scheduler_starvation,
        crate::failure_modes::security_policy_bypass,
        crate::failure_modes::update_brick,
        crate::failure_modes::use_after_free_kernel,
        crate::failure_modes::vm_escape,
    ],
    tags: ["cast_os_stdlib", "failure_modes"],
}

/// Sentinel for the failure_modes stdlib group.
pub struct FailureModesGroup;
