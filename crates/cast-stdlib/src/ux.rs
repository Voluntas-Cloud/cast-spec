//! Human, product & UX systems patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod accessibility_requirement;
pub mod action_preview;
pub mod attention_budgeting;
pub mod confirmation_for_destructive_action;
pub mod explainable_status;
pub mod human_in_the_loop_control;
pub mod localization_model;
pub mod notification_policy;
pub mod permission_prompt;
pub mod progressive_disclosure;
pub mod safe_default_action;
pub mod trust_calibration;
pub mod undoable_action;
pub mod user_intent_model;
pub mod user_preference_policy;

cast::concept! {
    name: "ux",
    summary: "Umbrella for the ux stdlib category. Human, product & UX \
              systems patterns.",
    anchors: [
        crate::ux::accessibility_requirement,
        crate::ux::action_preview,
        crate::ux::attention_budgeting,
        crate::ux::confirmation_for_destructive_action,
        crate::ux::explainable_status,
        crate::ux::human_in_the_loop_control,
        crate::ux::localization_model,
        crate::ux::notification_policy,
        crate::ux::permission_prompt,
        crate::ux::progressive_disclosure,
        crate::ux::safe_default_action,
        crate::ux::trust_calibration,
        crate::ux::undoable_action,
        crate::ux::user_intent_model,
        crate::ux::user_preference_policy,
    ],
    tags: ["cast_stdlib", "ux"],
}

/// Sentinel for the ux stdlib group.
pub struct UxGroup;

cast::rule! {
    rule: "Never make the user infer system state from vibes.",
    why:  "Humans already do enough superstition around computers. \
           Make state legible, decisions reversible, and destructive \
           actions explicitly confirmed; don't punish a misclick by \
           treating it as informed consent.",
    governs: [cast_stdlib::ux::UxGroup],
    tags: ["cast_stdlib", "ux"],
}
