//! `change_freeze` — period where releases are limited.

/// Sentinel for `change_freeze`.
pub struct ChangeFreeze;

cast::concept! {
    name: "change_freeze",
    summary: "Period where releases are limited. Around critical events \
              — Black Friday, an audit, a launch — the system stops \
              accepting non-essential changes; bug-fix-only mode.",
    anchors: [cast_stdlib::deployment::change_freeze::ChangeFreeze],
    tags: ["cast_stdlib", "deployment"],
}
