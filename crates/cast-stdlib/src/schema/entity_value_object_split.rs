//! `entity_value_object_split` — separate identity-bearing entities from pure values.

/// Sentinel for `entity_value_object_split`.
pub struct EntityValueObjectSplit;

cast::concept! {
    name: "entity_value_object_split",
    summary: "Separate identity-bearing entities from pure values. \
              Entities have a lifetime and ID; value objects are \
              equal-by-content and freely substitutable.",
    anchors: [cast_stdlib::schema::entity_value_object_split::EntityValueObjectSplit],
    tags: ["cast_stdlib", "schema"],
}
