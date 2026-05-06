//! `causation_id` — records which message caused another.

/// Sentinel for `causation_id`.
pub struct CausationId;

cast::concept! {
    name: "causation_id",
    summary: "Records which message caused another. Distinguishes the \
              sibling-of-each-other case (same correlation_id) from \
              the produced-by case.",
    anchors: [cast_stdlib::messaging::causation_id::CausationId],
    tags: ["cast_stdlib", "messaging"],
}
