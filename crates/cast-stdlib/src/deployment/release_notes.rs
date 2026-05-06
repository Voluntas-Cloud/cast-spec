//! `release_notes` — human-readable change summary.

/// Sentinel for `release_notes`.
pub struct ReleaseNotes;

cast::concept! {
    name: "release_notes",
    summary: "Human-readable change summary. Tells operators and \
              users what to expect — new behavior, breaking changes, \
              known issues — without reading the diff.",
    anchors: [cast_stdlib::deployment::release_notes::ReleaseNotes],
    tags: ["cast_stdlib", "deployment"],
}
