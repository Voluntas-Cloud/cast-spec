//! `distributed_tracing` — follow a request across services.

/// Sentinel for `distributed_tracing`.
pub struct DistributedTracing;

cast::concept! {
    name: "distributed_tracing",
    summary: "Follow request across services. Each span records its \
              segment of the request's path; assembled, the trace \
              shows where time and errors actually went.",
    anchors: [cast_stdlib::observability::distributed_tracing::DistributedTracing],
    tags: ["cast_stdlib", "observability"],
}
