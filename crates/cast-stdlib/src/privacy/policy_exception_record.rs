//! `policy_exception_record` — document deviations.

/// Sentinel for `policy_exception_record`.
pub struct PolicyExceptionRecord;

cast::concept! {
    name: "policy_exception_record",
    summary: "Document deviations. Every break from the standard \
              control has a recorded reason, owner, expiry, and \
              compensating measure; exceptions that don't expire are \
              just policy in denial.",
    anchors: [cast_stdlib::privacy::policy_exception_record::PolicyExceptionRecord],
    tags: ["cast_stdlib", "privacy"],
}
