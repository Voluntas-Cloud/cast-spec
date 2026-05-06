//! `tenant_isolation` — prevent cross-tenant data leakage.

/// Sentinel for `tenant_isolation`.
pub struct TenantIsolation;

cast::concept! {
    name: "tenant_isolation",
    summary: "Prevent cross-user or cross-org data leakage. The tenant \
              is part of every query, every cache key, every log line; \
              \"forgot to scope by tenant\" is the bug class that takes \
              down companies, not just features.",
    anchors: [cast_stdlib::privacy::tenant_isolation::TenantIsolation],
    tags: ["cast_stdlib", "privacy"],
}
