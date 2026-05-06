//! `privilege_ring_model` — hardware privilege levels such as ring 0 and ring 3.

/// Sentinel for `privilege_ring_model`.
pub struct PrivilegeRingModel;

cast::concept! {
    name: "privilege_ring_model",
    summary: "hardware privilege levels such as ring 0 and ring 3.",
    anchors: [cast_os_stdlib::kernel_user_boundary::privilege_ring_model::PrivilegeRingModel],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
