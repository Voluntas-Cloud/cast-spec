//! `trust_on_first_use` — cache identity at first connection.

/// Sentinel for `trust_on_first_use`.
pub struct TrustOnFirstUse;

cast::concept! {
    name: "trust_on_first_use",
    summary: "First observed identity is cached as trusted. Vulnerable \
              to MITM on first connection; resilient to MITM on \
              every subsequent connection.",
    anchors: [cast_stdlib::trust::trust_on_first_use::TrustOnFirstUse],
    tags: ["cast_stdlib", "trust"],
}
