//! `immutable_base_mutable_overlay_os` — read-only OS base plus writable layer.

/// Sentinel for `immutable_base_mutable_overlay_os`.
pub struct ImmutableBaseMutableOverlayOs;

cast::concept! {
    name: "immutable_base_mutable_overlay_os",
    summary: "read-only OS base plus writable layer.",
    anchors: [cast_os_stdlib::architectural_patterns::immutable_base_mutable_overlay_os::ImmutableBaseMutableOverlayOs],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
