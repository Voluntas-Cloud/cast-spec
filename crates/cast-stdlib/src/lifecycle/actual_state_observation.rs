//! `actual_state_observation` — record what is currently true.

/// Sentinel for `actual_state_observation`.
pub struct ActualStateObservation;

cast::concept! {
    name: "actual_state_observation",
    summary: "System records what is currently true. The 'is' to the \
              desired state's 'should be'; gap between them drives \
              the reconciler.",
    anchors: [cast_stdlib::lifecycle::actual_state_observation::ActualStateObservation],
    tags: ["cast_stdlib", "lifecycle"],
}
