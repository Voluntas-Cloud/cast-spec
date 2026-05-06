//! `polyrepo` — separate repos per project/service.

/// Sentinel for `polyrepo`.
pub struct Polyrepo;

cast::concept! {
    name: "polyrepo",
    summary: "Separate repositories per project or service. Each \
              repo gets independent release cadence and ownership; \
              cross-repo refactors get a tax that grows with the \
              number of repos involved.",
    anchors: [cast_stdlib::project_layout::polyrepo::Polyrepo],
    tags: ["cast_stdlib", "project_layout"],
}
