//! `brokered_access_model` — privileged broker mediates access.

/// Sentinel for `brokered_access_model`.
pub struct BrokeredAccessModel;

cast::concept! {
    name: "brokered_access_model",
    summary: "privileged broker mediates access.",
    anchors: [cast_os_stdlib::architectural_patterns::brokered_access_model::BrokeredAccessModel],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
