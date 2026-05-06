//! `source_of_truth_state` — canonical state.

/// Sentinel for `source_of_truth_state`.
pub struct SourceOfTruthState;

cast::concept! {
    name: "source_of_truth_state",
    summary: "The canonical state from which everything else \
              derives. The system has exactly one place where a \
              given fact is decided; \"both stores think they own \
              this field\" is how data corruption begins politely.",
    anchors: [cast_stdlib::state_data::source_of_truth_state::SourceOfTruthState],
    tags: ["cast_stdlib", "state_data"],
}
