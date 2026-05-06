//! `retention_policy` — define how long data lives.

/// Sentinel for `retention_policy`.
pub struct RetentionPolicy;

cast::concept! {
    name: "retention_policy",
    summary: "Define how long data lives. Each class has a TTL the \
              system enforces; without one, retention defaults to \
              \"forever\" and storage costs co-grow with regulatory \
              risk.",
    anchors: [cast_stdlib::privacy::retention_policy::RetentionPolicy],
    tags: ["cast_stdlib", "privacy"],
}
