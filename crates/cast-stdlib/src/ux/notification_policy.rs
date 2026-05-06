//! `notification_policy` — decide when/how to interrupt.

/// Sentinel for `notification_policy`.
pub struct NotificationPolicy;

cast::concept! {
    name: "notification_policy",
    summary: "Rules for when and how the system interrupts the user. \
              Every notification is a budget request from the user's \
              attention; spend it where it changes their behaviour, \
              not where it documents your zeal.",
    anchors: [cast_stdlib::ux::notification_policy::NotificationPolicy],
    tags: ["cast_stdlib", "ux"],
}
