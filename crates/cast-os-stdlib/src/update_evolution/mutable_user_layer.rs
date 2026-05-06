//! `mutable_user_layer` — user/app changes kept separate.

/// Sentinel for `mutable_user_layer`.
pub struct MutableUserLayer;

cast::concept! {
    name: "mutable_user_layer",
    summary: "user/app changes kept separate.",
    anchors: [cast_os_stdlib::update_evolution::mutable_user_layer::MutableUserLayer],
    tags: ["cast_os_stdlib", "update_evolution"],
}
