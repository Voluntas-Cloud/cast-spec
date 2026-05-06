//! `pseudonymization` — replace identity with reversible pseudonym.

/// Sentinel for `pseudonymization`.
pub struct Pseudonymization;

cast::concept! {
    name: "pseudonymization",
    summary: "Replace identity with a reversible or linked pseudonym. \
              Unlike anonymisation, the mapping is preserved (somewhere \
              colder, with stricter access) so the data can still be \
              reconciled when authorised.",
    anchors: [cast_stdlib::privacy::pseudonymization::Pseudonymization],
    tags: ["cast_stdlib", "privacy"],
}
