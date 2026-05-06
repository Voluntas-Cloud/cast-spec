//! `canonical_mapping` — map external concepts to internal model.

/// Sentinel for `canonical_mapping`.
pub struct CanonicalMapping;

cast::concept! {
    name: "canonical_mapping",
    summary: "An explicit translation from external concepts to the \
              internal canonical model. The mapping is code, \
              reviewable and testable; \"the developer who set this \
              up just knew what each field meant\" is not a mapping.",
    anchors: [cast_stdlib::integration::canonical_mapping::CanonicalMapping],
    tags: ["cast_stdlib", "integration"],
}
