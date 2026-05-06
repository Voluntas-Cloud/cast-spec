//! `explicit_nullability` — distinguish absence, null, empty.

/// Sentinel for `explicit_nullability`.
pub struct ExplicitNullability;

cast::concept! {
    name: "explicit_nullability",
    summary: "Field absence and null are intentionally modeled. The \
              schema distinguishes 'no value' from 'value is empty' \
              from 'value is unset'.",
    anchors: [cast_stdlib::schema::explicit_nullability::ExplicitNullability],
    tags: ["cast_stdlib", "schema"],
}
