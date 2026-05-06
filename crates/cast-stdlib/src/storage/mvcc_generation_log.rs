//! `mvcc_generation_log` — historical values keyed by generation.

/// Sentinel for `mvcc_generation_log`.
pub struct MvccGenerationLog;

cast::concept! {
    name: "mvcc_generation_log",
    summary: "Historical values keyed by generation/version. Reads are \
              parameterized by generation; earlier values remain \
              addressable. Substrate for snapshot isolation and \
              time-travel queries.",
    anchors: [cast_stdlib::storage::mvcc_generation_log::MvccGenerationLog],
    tags: ["cast_stdlib", "storage"],
}
