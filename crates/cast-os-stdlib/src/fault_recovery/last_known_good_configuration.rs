//! `last_known_good_configuration` — boot previous working config.

/// Sentinel for `last_known_good_configuration`.
pub struct LastKnownGoodConfiguration;

cast::concept! {
    name: "last_known_good_configuration",
    summary: "boot previous working config.",
    anchors: [cast_os_stdlib::fault_recovery::last_known_good_configuration::LastKnownGoodConfiguration],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
