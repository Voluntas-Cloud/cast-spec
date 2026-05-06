//! Output formatting for cast-extract.
//!
//! Three formats: `text` (the existing line-by-line CLI output),
//! `json` (structured, machine-consumable), `markdown` (PR-comment
//! shaped). All three render from the same `Report` value, so adding
//! a new format means writing one renderer — the loop in `main.rs`
//! doesn't move.

pub mod json;
pub mod markdown;
pub mod model;
pub mod text;

cast::concept! {
    name: "report_renderers",
    summary: "Pure render functions that turn a Report value into a \
              String in three shapes: text (line-oriented CLI), json \
              (machine-readable, sorted/stable for byte-identical CI \
              diffs), markdown (PR-comment shaped, ## section \
              headings). Each takes a &Report and returns a String — \
              no I/O, no mutation of the input, no time/RNG \
              dependency. Helpers append to a caller-owned String \
              buffer (a side effect on the buffer) but the overall \
              function is pure when viewed at the entry-point level.",
    anchors: [
        crate::emit::text::render,
        crate::emit::json::render,
        crate::emit::markdown::render,
    ],
    tags: ["cast_spec_emit"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "report_renderers",
    why: "Output is a function of (&Report) alone. No filesystem reads, \
          no clock, no RNG, no shared state. The same Report always \
          yields a byte-identical String.",
}

cast::concept! {
    name: "emit_format_choice",
    summary: "Which renderer is selected. Three-variant tag chosen by \
              the CLI's --format flag.",
    anchors: [
        crate::emit::Format,
    ],
    tags: ["cast_spec_emit"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "emit_format_choice",
    why: "Three-variant enum (Text, Json, Markdown); exactly one \
          inhabited per render call.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "emit_format_choice",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::integration::format_converter,
    concept: "report_renderers",
    why: lazy,
}

// One Report value, three renderers. The matrix pins the contract
// every renderer must satisfy: each format renders all three Report
// sections, just in its own shape. Adding a fourth format means
// writing one render() under emit/ and a Format variant — and
// extending this matrix with another row.
cast::matrix! {
    columns: [summary, per_invocation, concept_graph],
    rows: {
        text     @ crate::emit::text::render: [
            "leading invocation count line, then one summary block at \
             the bottom (counts of resolved/unresolved, parse errors, \
             pipeline errors, graph warnings)",
            "one line per invocation (`ok`/`ERR` prefix, macro path, \
             location), indented diagnostic lines beneath; CLI scripts \
             grep on these tokens",
            "flat block listing each concept with its declarations and \
             outgoing edges",
        ],
        json     @ crate::emit::json::render: [
            "Summary struct serialized as a top-level `summary` object \
             with stable field names",
            "groups: BTreeMap<tag, [InvocationReport]> — deterministic \
             ordering so byte-identical diff in CI",
            "concept_graph: ConceptGraphReport with concepts + warnings \
             arrays, ready for jq/structured consumers",
        ],
        markdown @ crate::emit::markdown::render: [
            "## Summary section with bullet list, designed to nest \
             under a wrapping `#` heading in a PR comment",
            "### per-invocation subsections grouped under ## Tag: \
             headings; tags become navigable jump targets",
            "## Concept graph section with bulleted concepts, each \
             with declarations and edges",
        ],
    },
    tags: ["emit_format"],
    note: "All three renderers consume the same `Report` value built \
           by `analysis_runner::build_report`; the format choice is a \
           pure render-time fan-out, not a separate analysis pass.",
}

pub use model::{
    kind_of, note_of, since_of, tags_of, AnnotationKind, ConceptGraphReport, InvocationReport,
    InvocationStatus, IoReport, PathReport, PipelineReportEntry, Report, Summary, UNTAGGED,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Text,
    Json,
    Markdown,
}

impl std::str::FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Format::Text),
            "json" => Ok(Format::Json),
            "markdown" | "md" => Ok(Format::Markdown),
            other => Err(format!(
                "unknown emit format `{other}` (expected text|json|markdown)"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Format;
    use std::str::FromStr;

    #[test]
    fn parses_canonical_names() {
        assert_eq!(Format::from_str("text").unwrap(), Format::Text);
        assert_eq!(Format::from_str("json").unwrap(), Format::Json);
        assert_eq!(Format::from_str("markdown").unwrap(), Format::Markdown);
    }

    #[test]
    fn md_alias_resolves_to_markdown() {
        assert_eq!(Format::from_str("md").unwrap(), Format::Markdown);
    }

    #[test]
    fn unknown_format_errors_with_helpful_message() {
        let err = Format::from_str("yaml").unwrap_err();
        assert!(err.contains("yaml"));
        assert!(err.contains("text|json|markdown"));
    }
}
