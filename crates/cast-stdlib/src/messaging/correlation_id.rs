//! `correlation_id` — ties related messages together.

/// Sentinel for `correlation_id`.
pub struct CorrelationId;

cast::concept! {
    name: "correlation_id",
    summary: "Ties related messages together. Lets you find every \
              message belonging to one logical operation in logs and \
              traces.",
    anchors: [cast_stdlib::messaging::correlation_id::CorrelationId],
    tags: ["cast_stdlib", "messaging"],
}
