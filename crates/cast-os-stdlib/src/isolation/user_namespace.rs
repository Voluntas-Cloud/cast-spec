//! `user_namespace` — isolate user/group identity mapping.

/// Sentinel for `user_namespace`.
pub struct UserNamespace;

cast::concept! {
    name: "user_namespace",
    summary: "isolate user/group identity mapping.",
    anchors: [cast_os_stdlib::isolation::user_namespace::UserNamespace],
    tags: ["cast_os_stdlib", "isolation"],
}
