//! `namespace_scoped_id` — uniqueness only inside a namespace.

/// Sentinel for `namespace_scoped_id`.
pub struct NamespaceScopedId;

cast::concept! {
    name: "namespace_scoped_id",
    summary: "Identity unique only inside a namespace. Same string \
              can name different things in different namespaces; \
              the namespace + ID together are global.",
    anchors: [cast_stdlib::identity::namespace_scoped_id::NamespaceScopedId],
    tags: ["cast_stdlib", "identity"],
}
