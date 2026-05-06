//! `reference_counting` — lifetime tracking by references.

/// Sentinel for `reference_counting`.
pub struct ReferenceCounting;

cast::concept! {
    name: "reference_counting",
    summary: "lifetime tracking by references.",
    anchors: [cast_os_stdlib::os_algorithms::reference_counting::ReferenceCounting],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
