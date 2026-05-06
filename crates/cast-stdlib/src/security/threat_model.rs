//! `threat_model` — explicit model of attackers and assets.

/// Sentinel for `threat_model`.
pub struct ThreatModel;

cast::concept! {
    name: "threat_model",
    summary: "Explicit model of attackers and assets. Who do we defend \
              against, what do they want, what do they already have? \
              Without it, security work optimises for the threats the \
              loudest engineer fears.",
    anchors: [cast_stdlib::security::threat_model::ThreatModel],
    tags: ["cast_stdlib", "security"],
}
