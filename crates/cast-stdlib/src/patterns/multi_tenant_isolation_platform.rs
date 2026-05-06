//! `multi_tenant_isolation_platform` — tenants are isolated by policy, storage, identity, and network.

/// Sentinel for `multi_tenant_isolation_platform`.
pub struct MultiTenantIsolationPlatform;

cast::concept! {
    name: "multi_tenant_isolation_platform",
    summary: "Multiple tenants, users, or projects are isolated by \
              policy, storage, identity, and network. Composes \
              tenant_isolation, namespace_scoped_id, resource_quota, \
              network_policy, secret_scoping, authorization_policy, \
              audit_log, and data_classification. Used for SaaS \
              platforms, hosted Voluntas services, shared AI \
              infrastructure, multi-user clusters, and development \
              environments.",
    anchors: [cast_stdlib::patterns::multi_tenant_isolation_platform::MultiTenantIsolationPlatform],
    tags: ["cast_stdlib", "patterns"],
}
