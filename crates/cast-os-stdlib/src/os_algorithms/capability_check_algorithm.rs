//! `capability_check_algorithm` — decide if handle grants authority.

/// Sentinel for `capability_check_algorithm`.
pub struct CapabilityCheckAlgorithm;

cast::concept! {
    name: "capability_check_algorithm",
    summary: "decide if handle grants authority.",
    anchors: [cast_os_stdlib::os_algorithms::capability_check_algorithm::CapabilityCheckAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
