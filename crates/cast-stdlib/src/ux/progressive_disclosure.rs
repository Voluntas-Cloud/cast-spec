//! `progressive_disclosure` — reveal complexity gradually.

/// Sentinel for `progressive_disclosure`.
pub struct ProgressiveDisclosure;

cast::concept! {
    name: "progressive_disclosure",
    summary: "Show the simple thing first; reveal the advanced \
              controls only when the user reaches for them. The screen \
              full of every option is honest about the system's \
              power, and dishonest about what most users need.",
    anchors: [cast_stdlib::ux::progressive_disclosure::ProgressiveDisclosure],
    tags: ["cast_stdlib", "ux"],
}
