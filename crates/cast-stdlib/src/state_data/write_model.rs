//! `write_model` — model optimized for commands/mutations.

/// Sentinel for `write_model`.
pub struct WriteModel;

cast::concept! {
    name: "write_model",
    summary: "A representation of state shaped for mutations: \
              normalised, validated, transactional. The write model \
              defines what changes are even allowed; the read model \
              is its echo.",
    anchors: [cast_stdlib::state_data::write_model::WriteModel],
    tags: ["cast_stdlib", "state_data"],
}
