//! `deprecation_lifecycle` — announce, support, warn, remove.

/// Sentinel for `deprecation_lifecycle`.
pub struct DeprecationLifecycle;

cast::concept! {
    name: "deprecation_lifecycle",
    summary: "Announce, support, warn, remove. Each phase has a \
              defined duration and exit criteria; surprise removals \
              are forbidden.",
    anchors: [cast_stdlib::lifecycle::deprecation_lifecycle::DeprecationLifecycle],
    tags: ["cast_stdlib", "lifecycle"],
}
