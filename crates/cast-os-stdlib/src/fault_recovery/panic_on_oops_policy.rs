//! `panic_on_oops_policy` — escalate kernel fault to panic.

/// Sentinel for `panic_on_oops_policy`.
pub struct PanicOnOopsPolicy;

cast::concept! {
    name: "panic_on_oops_policy",
    summary: "escalate kernel fault to panic.",
    anchors: [cast_os_stdlib::fault_recovery::panic_on_oops_policy::PanicOnOopsPolicy],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
