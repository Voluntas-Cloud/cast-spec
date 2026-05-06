//! `differential_publish` — publish only what changed plus an explicit clear for what disappeared.

/// Sentinel for `differential_publish`.
pub struct DifferentialPublish;

cast::concept! {
    name: "differential_publish",
    summary: "Each publish round computes the diff between the \
              previous published set and the next, and emits a \
              targeted update for each entry that changed PLUS an \
              explicit empty-or-clear message for each entry that \
              appeared in the previous round but not this one. \
              Without the explicit clear, fixed/removed items leave \
              stale state on the consumer forever. Bookkeeping (a \
              `last_published` map) is the cost; correctness on the \
              consumer side is the win.",
    anchors: [cast_stdlib::messaging::differential_publish::DifferentialPublish],
    tags: ["cast_stdlib", "messaging"],
}
