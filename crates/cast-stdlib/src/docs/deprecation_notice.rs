//! `deprecation_notice` — explain future removal.

/// Sentinel for `deprecation_notice`.
pub struct DeprecationNotice;

cast::concept! {
    name: "deprecation_notice",
    summary: "Tell callers a feature is going away, when, and what to \
              use instead. Removing without warning trains everyone to \
              pin to ancient versions and trust nothing.",
    anchors: [cast_stdlib::docs::deprecation_notice::DeprecationNotice],
    tags: ["cast_stdlib", "docs"],
}
