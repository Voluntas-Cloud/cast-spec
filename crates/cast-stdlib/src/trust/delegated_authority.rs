//! `delegated_authority` — one principal acts with limited rights from another.

/// Sentinel for `delegated_authority`.
pub struct DelegatedAuthority;

cast::concept! {
    name: "delegated_authority",
    summary: "One principal acts with limited rights from another. \
              The delegate cannot exceed the delegator's scope and \
              the action remains attributable to the delegator.",
    anchors: [cast_stdlib::trust::delegated_authority::DelegatedAuthority],
    tags: ["cast_stdlib", "trust"],
}
