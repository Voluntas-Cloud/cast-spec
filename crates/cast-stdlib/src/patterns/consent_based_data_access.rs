//! `consent_based_data_access` — sensitive data access depends on explicit consent and purpose.

/// Sentinel for `consent_based_data_access`.
pub struct ConsentBasedDataAccess;

cast::concept! {
    name: "consent_based_data_access",
    summary: "Access to sensitive data depends on explicit user \
              consent and a declared purpose. Composes \
              consent_record, purpose_limitation, \
              authorization_policy, user_preference_policy, \
              audit_log, data_classification, human_approval_step, \
              and revocable_credential. Used for AI reading personal \
              documents, finance/health integrations, sharing data \
              with services, mobile app permissions, and \
              user-controlled APIs.",
    anchors: [cast_stdlib::patterns::consent_based_data_access::ConsentBasedDataAccess],
    tags: ["cast_stdlib", "patterns"],
}
