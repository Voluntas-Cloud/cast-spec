//! `journaling_commit_algorithm` — commit durable filesystem transaction.

/// Sentinel for `journaling_commit_algorithm`.
pub struct JournalingCommitAlgorithm;

cast::concept! {
    name: "journaling_commit_algorithm",
    summary: "commit durable filesystem transaction.",
    anchors: [cast_os_stdlib::os_algorithms::journaling_commit_algorithm::JournalingCommitAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
