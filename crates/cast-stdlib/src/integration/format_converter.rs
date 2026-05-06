//! `format_converter` — translate file/message formats.

/// Sentinel for `format_converter`.
pub struct FormatConverter;

cast::concept! {
    name: "format_converter",
    summary: "Translate between file or message formats — CSV↔JSON, \
              XML↔Protobuf, Markdown↔HTML. Lives at the integration \
              boundary; resists the temptation to also normalise \
              semantics, which belongs in canonical_mapping.",
    anchors: [cast_stdlib::integration::format_converter::FormatConverter],
    tags: ["cast_stdlib", "integration"],
}
