//! `temporary_identifier` — provisional ID before durable assignment.

/// Sentinel for `temporary_identifier`.
pub struct TemporaryIdentifier;

cast::concept! {
    name: "temporary_identifier",
    summary: "Provisional identity before durable assignment. Lives \
              only until the durable ID is minted; references using \
              it are upgraded as part of finalization.",
    anchors: [cast_stdlib::identity::temporary_identifier::TemporaryIdentifier],
    tags: ["cast_stdlib", "identity"],
}
