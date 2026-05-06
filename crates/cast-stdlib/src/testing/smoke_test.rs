//! `smoke_test` — quick basic functionality check.

/// Sentinel for `smoke_test`.
pub struct SmokeTest;

cast::concept! {
    name: "smoke_test",
    summary: "Quick basic functionality check. Boots the system, hits \
              a few critical paths, fails fast if anything is \
              fundamentally broken — gates the rest of the suite.",
    anchors: [cast_stdlib::testing::smoke_test::SmokeTest],
    tags: ["cast_stdlib", "testing"],
}
