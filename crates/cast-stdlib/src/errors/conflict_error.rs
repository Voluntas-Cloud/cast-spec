//! `conflict_error` — state changed or conflicts.

/// Sentinel for `conflict_error`.
pub struct ConflictError;

cast::concept! {
    name: "conflict_error",
    summary: "State changed or conflicts. The expected precondition \
              (version, lock, uniqueness) no longer holds; the caller \
              must re-read, reconcile, and decide whether to retry.",
    anchors: [cast_stdlib::errors::conflict_error::ConflictError],
    tags: ["cast_stdlib", "errors"],
}
