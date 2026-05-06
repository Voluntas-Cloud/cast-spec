//! `model_checking` — explore state-machine behavior exhaustively.

/// Sentinel for `model_checking`.
pub struct ModelChecking;

cast::concept! {
    name: "model_checking",
    summary: "Explore state-machine behavior exhaustively. The model \
              checker walks every reachable state and verifies the \
              invariants hold; finds the corner cases hand-written \
              tests miss.",
    anchors: [cast_stdlib::testing::model_checking::ModelChecking],
    tags: ["cast_stdlib", "testing"],
}
