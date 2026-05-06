//! `dead_letter_queue` — failed messages set aside for inspection.

/// Sentinel for `dead_letter_queue`.
pub struct DeadLetterQueue;

cast::concept! {
    name: "dead_letter_queue",
    summary: "Failed messages sent aside for inspection. Keeps the main \
              queue moving while preserving the failed message for \
              human/automated diagnosis.",
    anchors: [cast_stdlib::messaging::dead_letter_queue::DeadLetterQueue],
    tags: ["cast_stdlib", "messaging"],
}
