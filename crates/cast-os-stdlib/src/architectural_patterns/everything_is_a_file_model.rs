//! `everything_is_a_file_model` — resources exposed through file-like interface.

/// Sentinel for `everything_is_a_file_model`.
pub struct EverythingIsAFileModel;

cast::concept! {
    name: "everything_is_a_file_model",
    summary: "resources exposed through file-like interface.",
    anchors: [cast_os_stdlib::architectural_patterns::everything_is_a_file_model::EverythingIsAFileModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
