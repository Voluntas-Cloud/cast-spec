//! `legal_hold` — preserve data due to legal requirement.

/// Sentinel for `legal_hold`.
pub struct LegalHold;

cast::concept! {
    name: "legal_hold",
    summary: "Preserve data because a legal or regulatory requirement \
              says we must. Holds override retention and erasure: the \
              normal pipelines have to know how to skip records that \
              are subject to a hold rather than silently violating it.",
    anchors: [cast_stdlib::privacy::legal_hold::LegalHold],
    tags: ["cast_stdlib", "privacy"],
}
