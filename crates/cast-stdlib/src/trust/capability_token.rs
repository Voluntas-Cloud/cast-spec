//! `capability_token` — bearer token grants a capability.

/// Sentinel for `capability_token`.
pub struct CapabilityToken;

cast::concept! {
    name: "capability_token",
    summary: "Bearer token grants a specific capability. The bearer \
              is the principal; possession alone is the proof.",
    anchors: [cast_stdlib::trust::capability_token::CapabilityToken],
    tags: ["cast_stdlib", "trust"],
}
