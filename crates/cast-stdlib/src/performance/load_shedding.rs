//! `load_shedding` — reject work to preserve health.

/// Sentinel for `load_shedding`.
pub struct LoadShedding;

cast::concept! {
    name: "load_shedding",
    summary: "Reject work to preserve health. When the system can't \
              keep up, dropping requests fast is better than queueing \
              them all and collapsing under the queue.",
    anchors: [cast_stdlib::performance::load_shedding::LoadShedding],
    tags: ["cast_stdlib", "performance"],
}
