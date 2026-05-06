//! `adapter_pattern` — translate one interface to another.

/// Sentinel for `adapter_pattern`.
pub struct AdapterPattern;

cast::concept! {
    name: "adapter_pattern",
    summary: "Translate one interface to another. Lets components \
              designed against different contracts work together.",
    anchors: [cast_stdlib::architecture::adapter_pattern::AdapterPattern],
    tags: ["cast_stdlib", "architecture"],
}
