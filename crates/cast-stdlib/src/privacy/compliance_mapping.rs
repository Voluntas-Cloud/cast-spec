//! `compliance_mapping` — map controls to legal/regulatory requirements.

/// Sentinel for `compliance_mapping`.
pub struct ComplianceMapping;

cast::concept! {
    name: "compliance_mapping",
    summary: "Map controls to legal or regulatory requirements. Each \
              code-enforced control carries the obligation it \
              satisfies; auditors and engineers see the same picture \
              instead of two parallel ones that diverge over time.",
    anchors: [cast_stdlib::privacy::compliance_mapping::ComplianceMapping],
    tags: ["cast_stdlib", "privacy"],
}
