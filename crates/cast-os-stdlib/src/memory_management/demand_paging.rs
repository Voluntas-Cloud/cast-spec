//! `demand_paging` — load pages when accessed.

/// Sentinel for `demand_paging`.
pub struct DemandPaging;

cast::concept! {
    name: "demand_paging",
    summary: "load pages when accessed.",
    anchors: [cast_os_stdlib::memory_management::demand_paging::DemandPaging],
    tags: ["cast_os_stdlib", "memory_management"],
}
