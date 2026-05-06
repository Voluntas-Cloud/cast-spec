//! `sandboxed_process` — process constrained by policy.

/// Sentinel for `sandboxed_process`.
pub struct SandboxedProcess;

cast::concept! {
    name: "sandboxed_process",
    summary: "process constrained by policy.",
    anchors: [cast_os_stdlib::security::sandboxed_process::SandboxedProcess],
    tags: ["cast_os_stdlib", "security"],
}
