//! `xarray_index` — Linux indexed pointer storage concept.

/// Sentinel for `xarray_index`.
pub struct XarrayIndex;

cast::concept! {
    name: "xarray_index",
    summary: "Linux indexed pointer storage concept.",
    anchors: [cast_os_stdlib::kernel_data_structures::xarray_index::XarrayIndex],
    tags: ["cast_os_stdlib", "kernel_data_structures"],
}
