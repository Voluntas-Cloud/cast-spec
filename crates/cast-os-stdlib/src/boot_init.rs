//! Boot, init, and system startup.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod boot_measurement_log;
pub mod boot_target_or_runlevel;
pub mod bootloader_stage;
pub mod dependency_based_boot_order;
pub mod early_kernel_init;
pub mod firmware_boot_stage;
pub mod init_process;
pub mod initramfs_stage;
pub mod kernel_command_line;
pub mod kernel_decompression;
pub mod launchd_service_model;
pub mod parallel_service_startup;
pub mod rc_script_init_model;
pub mod recovery_boot_mode;
pub mod root_filesystem_mount;
pub mod safe_mode_boot;
pub mod secure_boot_policy;
pub mod service_supervisor;
pub mod systemd_unit_model;
pub mod windows_service_control_manager;

cast::concept! {
    name: "boot_init",
    summary: "Umbrella for the boot_init stdlib category. Boot, init, and \
              system startup.",
    anchors: [
        crate::boot_init::boot_measurement_log,
        crate::boot_init::boot_target_or_runlevel,
        crate::boot_init::bootloader_stage,
        crate::boot_init::dependency_based_boot_order,
        crate::boot_init::early_kernel_init,
        crate::boot_init::firmware_boot_stage,
        crate::boot_init::init_process,
        crate::boot_init::initramfs_stage,
        crate::boot_init::kernel_command_line,
        crate::boot_init::kernel_decompression,
        crate::boot_init::launchd_service_model,
        crate::boot_init::parallel_service_startup,
        crate::boot_init::rc_script_init_model,
        crate::boot_init::recovery_boot_mode,
        crate::boot_init::root_filesystem_mount,
        crate::boot_init::safe_mode_boot,
        crate::boot_init::secure_boot_policy,
        crate::boot_init::service_supervisor,
        crate::boot_init::systemd_unit_model,
        crate::boot_init::windows_service_control_manager,
    ],
    tags: ["cast_os_stdlib", "boot_init"],
}

/// Sentinel for the boot_init stdlib group.
pub struct BootInitGroup;
