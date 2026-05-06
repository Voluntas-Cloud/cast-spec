//! `storage_stack_layering` — filesystem over block over device over driver. Beautifully cursed.

/// Sentinel for `storage_stack_layering`.
pub struct StorageStackLayering;

cast::concept! {
    name: "storage_stack_layering",
    summary: "filesystem over block over device over driver. Beautifully \
               cursed.",
    anchors: [cast_os_stdlib::filesystem_storage::storage_stack_layering::StorageStackLayering],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
