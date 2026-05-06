//! `driver_crash` — driver faults and destabilizes OS.

/// Sentinel for `driver_crash`.
pub struct DriverCrash;

cast::concept! {
    name: "driver_crash",
    summary: "driver faults and destabilizes OS.",
    anchors: [cast_os_stdlib::failure_modes::driver_crash::DriverCrash],
    tags: ["cast_os_stdlib", "failure_modes"],
}
