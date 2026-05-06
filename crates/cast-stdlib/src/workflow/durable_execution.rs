//! `durable_execution` — workflow survives process failure.

/// Sentinel for `durable_execution`.
pub struct DurableExecution;

cast::concept! {
    name: "durable_execution",
    summary: "Workflow survives process failure. The runtime journals \
              every effect; on crash a fresh worker replays the journal \
              and continues — the program reads as straight-line code \
              even though it spans hosts.",
    anchors: [cast_stdlib::workflow::durable_execution::DurableExecution],
    tags: ["cast_stdlib", "workflow"],
}
