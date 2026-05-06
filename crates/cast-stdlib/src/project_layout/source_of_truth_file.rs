//! `source_of_truth_file` — canonical file for derived artifacts.

/// Sentinel for `source_of_truth_file`.
pub struct SourceOfTruthFile;

cast::concept! {
    name: "source_of_truth_file",
    summary: "The canonical file from which derived artifacts come. \
              The build pipeline enforces the relationship; readers \
              know where to make a change and where not to bother.",
    anchors: [cast_stdlib::project_layout::source_of_truth_file::SourceOfTruthFile],
    tags: ["cast_stdlib", "project_layout"],
}
