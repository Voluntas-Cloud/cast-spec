//! `safe_failure_mode` — failure leaves system in acceptable state.

/// Sentinel for `safe_failure_mode`.
pub struct SafeFailureMode;

cast::concept! {
    name: "safe_failure_mode",
    summary: "Failure leaves the system in an acceptable state. Crashing \
              halfway can't violate invariants, lose acks, or strand \
              resources; the chosen failure mode (deny, hold, default) \
              is part of the design, not an emergent property.",
    anchors: [cast_stdlib::errors::safe_failure_mode::SafeFailureMode],
    tags: ["cast_stdlib", "errors"],
}
