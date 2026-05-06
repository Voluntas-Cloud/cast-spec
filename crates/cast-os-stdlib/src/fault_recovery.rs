//! Fault handling and recovery.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod automatic_repair_mode;
pub mod bugcheck_stop_error;
pub mod checkpoint_restore_process;
pub mod crash_only_service_design;
pub mod fail_stop_design;
pub mod fallback_kernel;
pub mod filesystem_recovery;
pub mod hard_lockup_detection;
pub mod hung_task_detection;
pub mod journal_replay;
pub mod kernel_panic;
pub mod kexec_fast_reboot;
pub mod last_known_good_configuration;
pub mod oom_recovery;
pub mod oops_recoverable_kernel_fault;
pub mod panic_on_oops_policy;
pub mod rollback_update;
pub mod safe_mode_recovery;
pub mod soft_lockup_detection;
pub mod watchdog_timer;

cast::concept! {
    name: "fault_recovery",
    summary: "Umbrella for the fault_recovery stdlib category. Fault \
              handling and recovery.",
    anchors: [
        crate::fault_recovery::automatic_repair_mode,
        crate::fault_recovery::bugcheck_stop_error,
        crate::fault_recovery::checkpoint_restore_process,
        crate::fault_recovery::crash_only_service_design,
        crate::fault_recovery::fail_stop_design,
        crate::fault_recovery::fallback_kernel,
        crate::fault_recovery::filesystem_recovery,
        crate::fault_recovery::hard_lockup_detection,
        crate::fault_recovery::hung_task_detection,
        crate::fault_recovery::journal_replay,
        crate::fault_recovery::kernel_panic,
        crate::fault_recovery::kexec_fast_reboot,
        crate::fault_recovery::last_known_good_configuration,
        crate::fault_recovery::oom_recovery,
        crate::fault_recovery::oops_recoverable_kernel_fault,
        crate::fault_recovery::panic_on_oops_policy,
        crate::fault_recovery::rollback_update,
        crate::fault_recovery::safe_mode_recovery,
        crate::fault_recovery::soft_lockup_detection,
        crate::fault_recovery::watchdog_timer,
    ],
    tags: ["cast_os_stdlib", "fault_recovery"],
}

/// Sentinel for the fault_recovery stdlib group.
pub struct FaultRecoveryGroup;
