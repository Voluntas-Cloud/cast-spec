//! `restart_policy` — rules for restarting failed service.

/// Sentinel for `restart_policy`.
pub struct RestartPolicy;

cast::concept! {
    name: "restart_policy",
    summary: "rules for restarting failed service.",
    anchors: [cast_os_stdlib::service_management::restart_policy::RestartPolicy],
    tags: ["cast_os_stdlib", "service_management"],
}
