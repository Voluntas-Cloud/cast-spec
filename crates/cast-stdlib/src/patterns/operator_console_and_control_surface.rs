//! `operator_console_and_control_surface` — humans inspect, approve, override, and repair system behavior.

/// Sentinel for `operator_console_and_control_surface`.
pub struct OperatorConsoleAndControlSurface;

cast::concept! {
    name: "operator_console_and_control_surface",
    summary: "Humans can understand, inspect, approve, override, \
              and repair system behavior. Composes \
              explainable_status, action_preview, \
              manual_intervention_hook, runbook, diagnostic_bundle, \
              audit_log, user_intent_model, and safe_default_action. \
              Used for admin dashboards, Voluntas Home, cluster \
              control UIs, AI action review panels, and incident \
              consoles.",
    anchors: [cast_stdlib::patterns::operator_console_and_control_surface::OperatorConsoleAndControlSurface],
    tags: ["cast_stdlib", "patterns"],
}
