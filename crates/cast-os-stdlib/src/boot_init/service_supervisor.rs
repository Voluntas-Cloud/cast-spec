//! `service_supervisor` — manages long-running services.

/// Sentinel for `service_supervisor`.
pub struct ServiceSupervisor;

cast::concept! {
    name: "service_supervisor",
    summary: "manages long-running services.",
    anchors: [cast_os_stdlib::boot_init::service_supervisor::ServiceSupervisor],
    tags: ["cast_os_stdlib", "boot_init"],
}
