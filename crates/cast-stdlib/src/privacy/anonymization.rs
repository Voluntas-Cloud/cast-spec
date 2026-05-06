//! `anonymization` — remove identifying info where possible.

/// Sentinel for `anonymization`.
pub struct Anonymization;

cast::concept! {
    name: "anonymization",
    summary: "Remove identifying information where possible. Real \
              anonymisation resists re-identification under realistic \
              auxiliary data; \"we removed the email\" is rarely \
              enough.",
    anchors: [cast_stdlib::privacy::anonymization::Anonymization],
    tags: ["cast_stdlib", "privacy"],
}
