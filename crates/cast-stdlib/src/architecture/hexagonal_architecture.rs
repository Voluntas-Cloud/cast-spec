//! `hexagonal_architecture` — domain core isolated from adapters.

/// Sentinel for `hexagonal_architecture`.
pub struct HexagonalArchitecture;

cast::concept! {
    name: "hexagonal_architecture",
    summary: "Domain core isolated from adapters. The core depends on \
              nothing external; adapters translate between the core \
              and the outside world.",
    anchors: [cast_stdlib::architecture::hexagonal_architecture::HexagonalArchitecture],
    tags: ["cast_stdlib", "architecture"],
}
