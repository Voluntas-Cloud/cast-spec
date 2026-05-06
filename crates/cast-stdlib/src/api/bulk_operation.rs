//! `bulk_operation` — large-scale operation with progress tracking.

/// Sentinel for `bulk_operation`.
pub struct BulkOperation;

cast::concept! {
    name: "bulk_operation",
    summary: "Large-scale operation with progress tracking. Different \
              from batch — these are long enough that the caller needs \
              status updates and may need to abort.",
    anchors: [cast_stdlib::api::bulk_operation::BulkOperation],
    tags: ["cast_stdlib", "api"],
}
