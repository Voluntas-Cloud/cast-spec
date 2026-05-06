//! `logging_journal` — structured service logs.

/// Sentinel for `logging_journal`.
pub struct LoggingJournal;

cast::concept! {
    name: "logging_journal",
    summary: "structured service logs.",
    anchors: [cast_os_stdlib::service_management::logging_journal::LoggingJournal],
    tags: ["cast_os_stdlib", "service_management"],
}
