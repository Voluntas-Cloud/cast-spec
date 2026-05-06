//! `monorepo` — many projects in one repo.

/// Sentinel for `monorepo`.
pub struct Monorepo;

cast::concept! {
    name: "monorepo",
    summary: "Many projects in one repository, sharing build, \
              tooling, and history. Trades cross-cutting refactor \
              ergonomics for coupling — when something in `core` \
              breaks, every consumer breaks at the same commit.",
    anchors: [cast_stdlib::project_layout::monorepo::Monorepo],
    tags: ["cast_stdlib", "project_layout"],
}
