//! `plain_file_configuration` — configuration stored in files.

/// Sentinel for `plain_file_configuration`.
pub struct PlainFileConfiguration;

cast::concept! {
    name: "plain_file_configuration",
    summary: "configuration stored in files.",
    anchors: [cast_os_stdlib::configuration::plain_file_configuration::PlainFileConfiguration],
    tags: ["cast_os_stdlib", "configuration"],
}
