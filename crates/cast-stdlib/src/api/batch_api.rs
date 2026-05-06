//! `batch_api` — combine multiple operations to reduce round-trips.

/// Sentinel for `batch_api`.
pub struct BatchApi;

cast::concept! {
    name: "batch_api",
    summary: "Combine multiple operations. Reduces round-trips; the \
              response shape needs to encode per-operation \
              success/failure.",
    anchors: [cast_stdlib::api::batch_api::BatchApi],
    tags: ["cast_stdlib", "api"],
}
