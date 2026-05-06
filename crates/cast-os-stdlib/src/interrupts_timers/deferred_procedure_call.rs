//! `deferred_procedure_call` — Windows-style deferred interrupt work.

/// Sentinel for `deferred_procedure_call`.
pub struct DeferredProcedureCall;

cast::concept! {
    name: "deferred_procedure_call",
    summary: "Windows-style deferred interrupt work.",
    anchors: [cast_os_stdlib::interrupts_timers::deferred_procedure_call::DeferredProcedureCall],
    tags: ["cast_os_stdlib", "interrupts_timers"],
}
