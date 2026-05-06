//! `create_update_delete_lifecycle` — explicit named state transitions.

/// Sentinel for `create_update_delete_lifecycle`.
pub struct CreateUpdateDeleteLifecycle;

cast::concept! {
    name: "create_update_delete_lifecycle",
    summary: "Explicit object lifecycle states. Each state transition \
              is named, gated, and logged; objects do not silently \
              jump from existing to gone.",
    anchors: [cast_stdlib::lifecycle::create_update_delete_lifecycle::CreateUpdateDeleteLifecycle],
    tags: ["cast_stdlib", "lifecycle"],
}
