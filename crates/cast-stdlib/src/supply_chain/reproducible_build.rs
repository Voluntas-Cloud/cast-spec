//! `reproducible_build` — same input produces same artifact.

/// Sentinel for `reproducible_build`.
pub struct ReproducibleBuild;

cast::concept! {
    name: "reproducible_build",
    summary: "Same input produces same artifact. Bit-for-bit, byte-for-\
              byte, regardless of who built it or when. Foundation for \
              independent verification of supply-chain integrity.",
    anchors: [cast_stdlib::supply_chain::reproducible_build::ReproducibleBuild],
    tags: ["cast_stdlib", "supply_chain"],
}
