//! `formatting_standard` — deterministic code formatting.

/// Sentinel for `formatting_standard`.
pub struct FormattingStandard;

cast::concept! {
    name: "formatting_standard",
    summary: "Deterministic formatting applied automatically. The \
              point is not the style, it's the absence of an opinion; \
              once the formatter runs in CI, nobody has to argue \
              about commas.",
    anchors: [cast_stdlib::project_layout::formatting_standard::FormattingStandard],
    tags: ["cast_stdlib", "project_layout"],
}
