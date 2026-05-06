//! `data_locality_scheduling` — place work near data.

/// Sentinel for `data_locality_scheduling`.
pub struct DataLocalityScheduling;

cast::concept! {
    name: "data_locality_scheduling",
    summary: "place work near data.",
    anchors: [cast_os_stdlib::distributed_os::data_locality_scheduling::DataLocalityScheduling],
    tags: ["cast_os_stdlib", "distributed_os"],
}
