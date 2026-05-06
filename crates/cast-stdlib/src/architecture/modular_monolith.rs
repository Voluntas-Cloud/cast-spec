//! `modular_monolith` — one artifact, deliberate internal APIs.

/// Sentinel for `modular_monolith`.
pub struct ModularMonolith;

cast::concept! {
    name: "modular_monolith",
    summary: "Monolith with strict internal boundaries. One artifact, \
              but modules talk to each other only through deliberate APIs.",
    anchors: [cast_stdlib::architecture::modular_monolith::ModularMonolith],
    tags: ["cast_stdlib", "architecture"],
}
