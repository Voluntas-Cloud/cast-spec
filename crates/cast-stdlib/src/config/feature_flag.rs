//! `feature_flag` — enable/disable behavior without redeploy.

/// Sentinel for `feature_flag`.
pub struct FeatureFlag;

cast::concept! {
    name: "feature_flag",
    summary: "Enable/disable behavior without redeploy. Lets risky \
              changes ship dark, get rolled out gradually, and be \
              turned off instantly when something goes wrong.",
    anchors: [cast_stdlib::config::feature_flag::FeatureFlag],
    tags: ["cast_stdlib", "config"],
}
