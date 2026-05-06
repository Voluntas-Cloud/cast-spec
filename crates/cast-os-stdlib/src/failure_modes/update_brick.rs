//! `update_brick` — update renders device unusable. Delightful phrase, horrifying reality.

/// Sentinel for `update_brick`.
pub struct UpdateBrick;

cast::concept! {
    name: "update_brick",
    summary: "update renders device unusable. Delightful phrase, \
               horrifying reality.",
    anchors: [cast_os_stdlib::failure_modes::update_brick::UpdateBrick],
    tags: ["cast_os_stdlib", "failure_modes"],
}
