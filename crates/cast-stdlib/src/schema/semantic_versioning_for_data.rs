//! `semantic_versioning_for_data` — major/minor/patch by compatibility impact.

/// Sentinel for `semantic_versioning_for_data`.
pub struct SemanticVersioningForData;

cast::concept! {
    name: "semantic_versioning_for_data",
    summary: "Version meaning by compatibility impact. Major bump = \
              breaking; minor = additive; patch = cosmetic. Same idea \
              as code, applied to schemas.",
    anchors: [cast_stdlib::schema::semantic_versioning_for_data::SemanticVersioningForData],
    tags: ["cast_stdlib", "schema"],
}
