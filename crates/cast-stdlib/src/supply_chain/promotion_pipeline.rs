//! `promotion_pipeline` — same artifact moves through environments.

/// Sentinel for `promotion_pipeline`.
pub struct PromotionPipeline;

cast::concept! {
    name: "promotion_pipeline",
    summary: "Same artifact moves through environments. Build once \
              in CI, then promote that exact artifact through dev, \
              staging, production — never rebuild for promotion.",
    anchors: [cast_stdlib::supply_chain::promotion_pipeline::PromotionPipeline],
    tags: ["cast_stdlib", "supply_chain"],
}
