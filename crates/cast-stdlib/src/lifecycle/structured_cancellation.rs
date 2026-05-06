//! `structured_cancellation` — cooperative cancellation that propagates through async tasks.

/// Sentinel for `structured_cancellation`.
pub struct StructuredCancellation;

cast::concept! {
    name: "structured_cancellation",
    summary: "Cancellation is a first-class signal that flows down the \
              call tree alongside the work. A token (or scope) handed \
              to a callee lets the caller revoke the work cleanly: \
              the callee polls the token between steps, releases its \
              resources, and returns. Distinct from killing a thread \
              or aborting a future — those leave locks held, \
              destructors unrun, and the caller no way to know how \
              far the callee got.",
    anchors: [cast_stdlib::lifecycle::structured_cancellation::StructuredCancellation],
    tags: ["cast_stdlib", "lifecycle"],
}
