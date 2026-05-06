//! `right_to_erasure_workflow` — delete personal data when required.

/// Sentinel for `right_to_erasure_workflow`.
pub struct RightToErasureWorkflow;

cast::concept! {
    name: "right_to_erasure_workflow",
    summary: "Delete personal data when required. A first-class \
              operation that touches every store, replica, backup, and \
              index where the subject's data lives — the system already \
              knows the map; the workflow doesn't reinvent it under \
              deadline.",
    anchors: [cast_stdlib::privacy::right_to_erasure_workflow::RightToErasureWorkflow],
    tags: ["cast_stdlib", "privacy"],
}
