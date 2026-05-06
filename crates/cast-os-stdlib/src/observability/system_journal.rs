//! `system_journal` — structured OS/service log store.

/// Sentinel for `system_journal`.
pub struct SystemJournal;

cast::concept! {
    name: "system_journal",
    summary: "structured OS/service log store.",
    anchors: [cast_os_stdlib::observability::system_journal::SystemJournal],
    tags: ["cast_os_stdlib", "observability"],
}
