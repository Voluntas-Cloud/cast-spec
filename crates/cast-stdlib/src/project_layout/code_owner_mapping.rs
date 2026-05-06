//! `code_owner_mapping` — ownership by path/module.

/// Sentinel for `code_owner_mapping`.
pub struct CodeOwnerMapping;

cast::concept! {
    name: "code_owner_mapping",
    summary: "Ownership rules by path or module — who reviews, who \
              gets paged, who decides direction. Without it, \
              ownership becomes implicit and migrates to whoever \
              answered the last question about that file.",
    anchors: [cast_stdlib::project_layout::code_owner_mapping::CodeOwnerMapping],
    tags: ["cast_stdlib", "project_layout"],
}
