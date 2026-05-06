//! `character_device_layer` — stream-like device abstraction.

/// Sentinel for `character_device_layer`.
pub struct CharacterDeviceLayer;

cast::concept! {
    name: "character_device_layer",
    summary: "stream-like device abstraction.",
    anchors: [cast_os_stdlib::filesystem_storage::character_device_layer::CharacterDeviceLayer],
    tags: ["cast_os_stdlib", "filesystem_storage"],
}
