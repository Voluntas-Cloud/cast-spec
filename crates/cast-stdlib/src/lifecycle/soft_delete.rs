//! `soft_delete` — mark deleted, keep recoverable.

/// Sentinel for `soft_delete`.
pub struct SoftDelete;

cast::concept! {
    name: "soft_delete",
    summary: "Mark deleted without immediate removal. Recoverable, \
              auditable, and safer for accidental deletion; cost is \
              storage and the discipline to filter at every read.",
    anchors: [cast_stdlib::lifecycle::soft_delete::SoftDelete],
    tags: ["cast_stdlib", "lifecycle"],
}
