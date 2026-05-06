//! `consent_record` — durable proof of user consent.

/// Sentinel for `consent_record`.
pub struct ConsentRecord;

cast::concept! {
    name: "consent_record",
    summary: "Durable proof of user consent. What they agreed to, when, \
              under which version of the policy, and through what \
              interface — recorded so processing decisions can cite the \
              specific basis.",
    anchors: [cast_stdlib::privacy::consent_record::ConsentRecord],
    tags: ["cast_stdlib", "privacy"],
}
