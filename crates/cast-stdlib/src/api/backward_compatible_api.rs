//! `backward_compatible_api` — existing clients keep working through evolution.

/// Sentinel for `backward_compatible_api`.
pub struct BackwardCompatibleApi;

cast::concept! {
    name: "backward_compatible_api",
    summary: "Existing clients keep working when the API evolves. \
              Adding optional inputs and outputs; never removing or \
              repurposing.",
    anchors: [cast_stdlib::api::backward_compatible_api::BackwardCompatibleApi],
    tags: ["cast_stdlib", "api"],
}
