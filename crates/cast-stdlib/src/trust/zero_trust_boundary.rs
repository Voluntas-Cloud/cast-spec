//! `zero_trust_boundary` — every request verified regardless of network location.

/// Sentinel for `zero_trust_boundary`.
pub struct ZeroTrustBoundary;

cast::concept! {
    name: "zero_trust_boundary",
    summary: "Every request is verified regardless of network \
              location. The internal network is no more trusted than \
              the public internet.",
    anchors: [cast_stdlib::trust::zero_trust_boundary::ZeroTrustBoundary],
    tags: ["cast_stdlib", "trust"],
}
