//! `domain_event_automation_system` — domain events trigger policies, workflows, AI assistance.

/// Sentinel for `domain_event_automation_system`.
pub struct DomainEventAutomationSystem;

cast::concept! {
    name: "domain_event_automation_system",
    summary: "Domain events trigger policies, workflows, \
              notifications, and AI assistance. Composes \
              event_message, publish_subscribe, policy_as_code, \
              event_triggered_job, workflow_orchestration, \
              notification_policy, human_approval_step, and \
              audit_log. Used to translate \"invoice detected\" → \
              finance workflow, \"email received\" → \
              classify/action, \"node degraded\" → repair, \"new \
              document\" → index/summarise, and so on.",
    anchors: [cast_stdlib::patterns::domain_event_automation_system::DomainEventAutomationSystem],
    tags: ["cast_stdlib", "patterns"],
}
