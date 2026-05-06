//! `service_boundary` — runtime boundary around an independent capability.

/// Sentinel for `service_boundary`.
pub struct ServiceBoundary;

cast::concept! {
    name: "service_boundary",
    summary: "Runtime boundary around independent capability. Crossed by \
              network calls; failures and latencies live here.",
    anchors: [cast_stdlib::architecture::service_boundary::ServiceBoundary],
    tags: ["cast_stdlib", "architecture"],
}
