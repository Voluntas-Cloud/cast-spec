//! `feature_lifecycle` — experimental, beta, stable, deprecated.

/// Sentinel for `feature_lifecycle`.
pub struct FeatureLifecycle;

cast::concept! {
    name: "feature_lifecycle",
    summary: "Experimental, beta, stable, deprecated. Each tier has a \
              support contract; users read the tier and know what to \
              expect.",
    anchors: [cast_stdlib::lifecycle::feature_lifecycle::FeatureLifecycle],
    tags: ["cast_stdlib", "lifecycle"],
}
