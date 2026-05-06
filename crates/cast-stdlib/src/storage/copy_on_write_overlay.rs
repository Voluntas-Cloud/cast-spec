//! `copy_on_write_overlay` — new layer on write, structural sharing of unchanged subtrees.

/// Sentinel for `copy_on_write_overlay`.
pub struct CopyOnWriteOverlay;

cast::concept! {
    name: "copy_on_write_overlay",
    summary: "Changes are written to a new layer instead of mutating \
              the base. Versions share unchanged subtrees by reference; \
              memory cost tracks churn rate, not version count.",
    anchors: [cast_stdlib::storage::copy_on_write_overlay::CopyOnWriteOverlay],
    tags: ["cast_stdlib", "storage"],
}
