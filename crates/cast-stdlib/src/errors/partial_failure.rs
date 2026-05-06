//! `partial_failure` — some sub-operations failed.

/// Sentinel for `partial_failure`.
pub struct PartialFailure;

cast::concept! {
    name: "partial_failure",
    summary: "Some sub-operations failed. A bulk request returns a \
              per-item outcome list; the overall envelope is success \
              only when the response also tells the caller exactly \
              which items did and didn't apply.",
    anchors: [cast_stdlib::errors::partial_failure::PartialFailure],
    tags: ["cast_stdlib", "errors"],
}
