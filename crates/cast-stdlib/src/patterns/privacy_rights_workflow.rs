//! `privacy_rights_workflow` — personal data is classified, exported, deleted, retained, or held.

/// Sentinel for `privacy_rights_workflow`.
pub struct PrivacyRightsWorkflow;

cast::concept! {
    name: "privacy_rights_workflow",
    summary: "Personal data can be classified, exported, deleted, \
              retained, or put under legal hold. Composes \
              data_classification, retention_policy, \
              right_to_erasure_workflow, consent_record, \
              data_lineage, audit_trail, purpose_limitation, and \
              legal_hold. Used for GDPR compliance, personal data \
              management, user export/delete flows, privacy-aware \
              automation, and regulated applications.",
    anchors: [cast_stdlib::patterns::privacy_rights_workflow::PrivacyRightsWorkflow],
    tags: ["cast_stdlib", "patterns"],
}
