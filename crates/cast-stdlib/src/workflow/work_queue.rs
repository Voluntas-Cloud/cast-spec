//! `work_queue` — workers pull available tasks.

/// Sentinel for `work_queue`.
pub struct WorkQueue;

cast::concept! {
    name: "work_queue",
    summary: "Workers pull available tasks. Producers do not address \
              specific consumers; capacity is added by spawning more \
              workers, and slow consumers naturally apply backpressure \
              by not pulling.",
    anchors: [cast_stdlib::workflow::work_queue::WorkQueue],
    tags: ["cast_stdlib", "workflow"],
}
