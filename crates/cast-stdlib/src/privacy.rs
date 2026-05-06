//! Privacy, compliance & governance patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod access_review;
pub mod anonymization;
pub mod audit_trail;
pub mod compliance_mapping;
pub mod consent_record;
pub mod data_classification;
pub mod data_lineage;
pub mod legal_hold;
pub mod policy_exception_record;
pub mod privacy_by_design;
pub mod pseudonymization;
pub mod purpose_limitation;
pub mod retention_policy;
pub mod right_to_erasure_workflow;
pub mod tenant_isolation;

cast::concept! {
    name: "privacy",
    summary: "Umbrella for the privacy stdlib category. Privacy, \
              compliance & governance patterns.",
    anchors: [
        crate::privacy::access_review,
        crate::privacy::anonymization,
        crate::privacy::audit_trail,
        crate::privacy::compliance_mapping,
        crate::privacy::consent_record,
        crate::privacy::data_classification,
        crate::privacy::data_lineage,
        crate::privacy::legal_hold,
        crate::privacy::policy_exception_record,
        crate::privacy::privacy_by_design,
        crate::privacy::pseudonymization,
        crate::privacy::purpose_limitation,
        crate::privacy::retention_policy,
        crate::privacy::right_to_erasure_workflow,
        crate::privacy::tenant_isolation,
    ],
    tags: ["cast_stdlib", "privacy"],
}

/// Sentinel for the privacy stdlib group.
pub struct PrivacyGroup;

cast::rule! {
    rule: "Compliance should be encoded into workflows.",
    why:  "Not stapled on by a doomed spreadsheet priest. Controls \
           that live as code run automatically; controls that live as \
           prose run only when someone remembers to read them.",
    governs: [cast_stdlib::privacy::PrivacyGroup],
    tags: ["cast_stdlib", "privacy"],
}
