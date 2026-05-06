//! `read_model` — optimized view for querying.

/// Sentinel for `read_model`.
pub struct ReadModel;

cast::concept! {
    name: "read_model",
    summary: "A representation of state shaped for queries: \
              denormalised, indexed, sometimes summary-only. The \
              read model can lie about freshness in exchange for \
              speed, as long as the lie is bounded and documented.",
    anchors: [cast_stdlib::state_data::read_model::ReadModel],
    tags: ["cast_stdlib", "state_data"],
}
