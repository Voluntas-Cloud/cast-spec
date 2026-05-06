//! `filter_driver` — driver intercepting/modifying requests.

/// Sentinel for `filter_driver`.
pub struct FilterDriver;

cast::concept! {
    name: "filter_driver",
    summary: "driver intercepting/modifying requests.",
    anchors: [cast_os_stdlib::driver_model::filter_driver::FilterDriver],
    tags: ["cast_os_stdlib", "driver_model"],
}
