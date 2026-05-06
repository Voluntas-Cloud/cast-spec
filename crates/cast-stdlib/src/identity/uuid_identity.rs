//! `uuid_identity` — globally unique without coordination.

/// Sentinel for `uuid_identity`.
pub struct UuidIdentity;

cast::concept! {
    name: "uuid_identity",
    summary: "Globally unique identifier without central coordination. \
              Trades the tight density of monotonic IDs for the \
              freedom to mint identifiers from anywhere without \
              talking to a central authority.",
    anchors: [cast_stdlib::identity::uuid_identity::UuidIdentity],
    tags: ["cast_stdlib", "identity"],
}
