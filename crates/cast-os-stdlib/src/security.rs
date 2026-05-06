//! Security model concepts.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod access_control_list;
pub mod apparmor_profile_model;
pub mod audit_subsystem;
pub mod capability_based_security;
pub mod code_signing_enforcement;
pub mod credential_cache;
pub mod discretionary_access_control;
pub mod keyring_subsystem;
pub mod least_privilege_process_model;
pub mod linux_capabilities;
pub mod lsm_hook_model;
pub mod mandatory_access_control;
pub mod mandatory_integrity_control;
pub mod measured_boot_chain;
pub mod pledge_unveil_model;
pub mod posix_acl;
pub mod privilege_separation;
pub mod remote_attestation;
pub mod role_based_access_control;
pub mod sandboxed_process;
pub mod seccomp_syscall_filtering;
pub mod secure_boot_chain;
pub mod security_descriptor;
pub mod selinux_policy_model;
pub mod setuid_execution;
pub mod trusted_platform_module;
pub mod unix_permission_bits;

cast::concept! {
    name: "security",
    summary: "Umbrella for the security stdlib category. Security model \
              concepts.",
    anchors: [
        crate::security::access_control_list,
        crate::security::apparmor_profile_model,
        crate::security::audit_subsystem,
        crate::security::capability_based_security,
        crate::security::code_signing_enforcement,
        crate::security::credential_cache,
        crate::security::discretionary_access_control,
        crate::security::keyring_subsystem,
        crate::security::least_privilege_process_model,
        crate::security::linux_capabilities,
        crate::security::lsm_hook_model,
        crate::security::mandatory_access_control,
        crate::security::mandatory_integrity_control,
        crate::security::measured_boot_chain,
        crate::security::pledge_unveil_model,
        crate::security::posix_acl,
        crate::security::privilege_separation,
        crate::security::remote_attestation,
        crate::security::role_based_access_control,
        crate::security::sandboxed_process,
        crate::security::seccomp_syscall_filtering,
        crate::security::secure_boot_chain,
        crate::security::security_descriptor,
        crate::security::selinux_policy_model,
        crate::security::setuid_execution,
        crate::security::trusted_platform_module,
        crate::security::unix_permission_bits,
    ],
    tags: ["cast_os_stdlib", "security"],
}

/// Sentinel for the security stdlib group.
pub struct SecurityGroup;
