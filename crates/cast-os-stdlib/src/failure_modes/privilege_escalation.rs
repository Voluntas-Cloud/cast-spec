//! `privilege_escalation` — lower-privileged actor gains higher rights.

/// Sentinel for `privilege_escalation`.
pub struct PrivilegeEscalation;

cast::concept! {
    name: "privilege_escalation",
    summary: "lower-privileged actor gains higher rights.",
    anchors: [cast_os_stdlib::failure_modes::privilege_escalation::PrivilegeEscalation],
    tags: ["cast_os_stdlib", "failure_modes"],
}
