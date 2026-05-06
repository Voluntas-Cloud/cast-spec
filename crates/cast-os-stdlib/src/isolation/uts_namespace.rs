//! `uts_namespace` — isolate hostname/domain identity.

/// Sentinel for `uts_namespace`.
pub struct UtsNamespace;

cast::concept! {
    name: "uts_namespace",
    summary: "isolate hostname/domain identity.",
    anchors: [cast_os_stdlib::isolation::uts_namespace::UtsNamespace],
    tags: ["cast_os_stdlib", "isolation"],
}
