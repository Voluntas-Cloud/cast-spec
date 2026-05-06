//! `journal_replay` — apply filesystem/storage journal after crash.

/// Sentinel for `journal_replay`.
pub struct JournalReplay;

cast::concept! {
    name: "journal_replay",
    summary: "apply filesystem/storage journal after crash.",
    anchors: [cast_os_stdlib::fault_recovery::journal_replay::JournalReplay],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
