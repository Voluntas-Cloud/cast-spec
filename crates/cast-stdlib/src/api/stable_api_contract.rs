//! `stable_api_contract` — external behavior promised across versions.

/// Sentinel for `stable_api_contract`.
pub struct StableApiContract;

cast::concept! {
    name: "stable_api_contract",
    summary: "External behavior promised across versions. Inputs, \
              outputs, errors, and side effects that callers can rely on.",
    anchors: [cast_stdlib::api::stable_api_contract::StableApiContract],
    tags: ["cast_stdlib", "api"],
}
