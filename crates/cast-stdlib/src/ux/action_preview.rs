//! `action_preview` — show effect before execution.

/// Sentinel for `action_preview`.
pub struct ActionPreview;

cast::concept! {
    name: "action_preview",
    summary: "Show what the action will do before it happens — diff, \
              dry-run, planned changes. Once the action runs, the \
              preview is the only thing that lets the user catch a \
              mistake before it lands.",
    anchors: [cast_stdlib::ux::action_preview::ActionPreview],
    tags: ["cast_stdlib", "ux"],
}
