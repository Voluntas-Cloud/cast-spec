//! `trap_gate` — hardware-supported transition into privileged code.

/// Sentinel for `trap_gate`.
pub struct TrapGate;

cast::concept! {
    name: "trap_gate",
    summary: "hardware-supported transition into privileged code.",
    anchors: [cast_os_stdlib::kernel_user_boundary::trap_gate::TrapGate],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
