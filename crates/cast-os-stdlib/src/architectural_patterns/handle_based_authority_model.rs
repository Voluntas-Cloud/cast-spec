//! `handle_based_authority_model` — access through unforgeable handles.

/// Sentinel for `handle_based_authority_model`.
pub struct HandleBasedAuthorityModel;

cast::concept! {
    name: "handle_based_authority_model",
    summary: "access through unforgeable handles.",
    anchors: [cast_os_stdlib::architectural_patterns::handle_based_authority_model::HandleBasedAuthorityModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
