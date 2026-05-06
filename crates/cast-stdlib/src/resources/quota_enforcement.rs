//! `quota_enforcement` — cap usage by tenant/user/group.

/// Sentinel for `quota_enforcement`.
pub struct QuotaEnforcement;

cast::concept! {
    name: "quota_enforcement",
    summary: "Cap usage by tenant, user, or group. A noisy neighbour \
              cannot consume capacity that belongs to a quieter one; \
              the quota check happens at admission time, not after the \
              fact.",
    anchors: [cast_stdlib::resources::quota_enforcement::QuotaEnforcement],
    tags: ["cast_stdlib", "resources"],
}
