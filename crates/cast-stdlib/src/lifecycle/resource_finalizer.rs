//! `resource_finalizer` — cleanup hook before deletion completes.

/// Sentinel for `resource_finalizer`.
pub struct ResourceFinalizer;

cast::concept! {
    name: "resource_finalizer",
    summary: "Cleanup hook before deletion completes. Releases \
              external resources, frees quota, notifies dependents — \
              guaranteed to run before the resource is gone.",
    anchors: [cast_stdlib::lifecycle::resource_finalizer::ResourceFinalizer],
    tags: ["cast_stdlib", "lifecycle"],
}
