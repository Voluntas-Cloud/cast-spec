//! `tamper_evidence` — modification can be detected.

/// Sentinel for `tamper_evidence`.
pub struct TamperEvidence;

cast::concept! {
    name: "tamper_evidence",
    summary: "Modification can be detected. Hashes, signatures, or \
              chained logs make silent edits impossible; the system \
              may not prevent tampering but it always notices.",
    anchors: [cast_stdlib::security::tamper_evidence::TamperEvidence],
    tags: ["cast_stdlib", "security"],
}
