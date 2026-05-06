//! `least_privilege` — only the permissions required.

/// Sentinel for `least_privilege`.
pub struct LeastPrivilege;

cast::concept! {
    name: "least_privilege",
    summary: "Grant only the permissions required. Over-broad grants \
              are the leading cause of catastrophic breach blast \
              radius.",
    anchors: [cast_stdlib::trust::least_privilege::LeastPrivilege],
    tags: ["cast_stdlib", "trust"],
}
