//! `time_bound_credential` — credential expires after defined period.

/// Sentinel for `time_bound_credential`.
pub struct TimeBoundCredential;

cast::concept! {
    name: "time_bound_credential",
    summary: "Credential expires after a defined period. Limits the \
              window of exploitation if the credential leaks.",
    anchors: [cast_stdlib::trust::time_bound_credential::TimeBoundCredential],
    tags: ["cast_stdlib", "trust"],
}
