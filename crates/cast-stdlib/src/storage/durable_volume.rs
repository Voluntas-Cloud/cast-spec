//! `durable_volume` — persistence independent of process/container lifetime.

/// Sentinel for `durable_volume`.
pub struct DurableVolume;

cast::concept! {
    name: "durable_volume",
    summary: "Persisted storage independent of process/container \
              lifetime. Survives restarts, redeploys, and rescheduling \
              of the consumer.",
    anchors: [cast_stdlib::storage::durable_volume::DurableVolume],
    tags: ["cast_stdlib", "storage"],
}
