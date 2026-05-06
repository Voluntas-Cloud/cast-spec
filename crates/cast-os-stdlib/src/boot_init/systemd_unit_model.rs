//! `systemd_unit_model` — dependency-based service/resource units.

/// Sentinel for `systemd_unit_model`.
pub struct SystemdUnitModel;

cast::concept! {
    name: "systemd_unit_model",
    summary: "dependency-based service/resource units.",
    anchors: [cast_os_stdlib::boot_init::systemd_unit_model::SystemdUnitModel],
    tags: ["cast_os_stdlib", "boot_init"],
}
