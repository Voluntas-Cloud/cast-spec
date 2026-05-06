//! `thermal_zone_model` — represent heat domains.

/// Sentinel for `thermal_zone_model`.
pub struct ThermalZoneModel;

cast::concept! {
    name: "thermal_zone_model",
    summary: "represent heat domains.",
    anchors: [cast_os_stdlib::power_thermal::thermal_zone_model::ThermalZoneModel],
    tags: ["cast_os_stdlib", "power_thermal"],
}
