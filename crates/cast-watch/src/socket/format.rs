use super::protocol::{OutputFormat, ResponseBody};

cast::matrix! {
    columns: [name_only, summary, full],
    rows: {
        json @ crate::socket::OutputFormat::Json: [
            "JSON list, names only — minimum bytes per hit, fastest to scan.",
            "JSON with per-hit summary fields — middle ground for most LLM context.",
            "JSON with full record bodies — legacy structured response shape.",
        ],
        yaml @ crate::socket::OutputFormat::Yaml: [
            "YAML name list — flat, human-skimmable, smallest token cost.",
            "YAML with summaries — compact prose-shaped overview.",
            "YAML with full bodies — full detail in line-oriented form.",
        ],
    },
    tags: ["cast_watch"],
    note: "The 6-cell decision matrix output_format prose describes. \
           Rows = wire format (json | yaml); columns = dim brightness \
           (name_only | summary | full). Both axes compose orthogonally \
           — pick one cell per query.",
}

cast::concept! {
    name: "output_format",
    summary: "Wire-format negotiation for query/walk responses, \
              orthogonal to flashlight's `dim`. `dim` selects WHAT \
              data is included in each hit; `output_format` selects \
              HOW that data is encoded for the wire. The two compose \
              into a 6-cell matrix of useful response shapes from \
              one query: e.g. `dim: name_only, format: yaml` is a \
              flat YAML list a human or LLM can skim; `dim: full, \
              format: json` is the legacy structured response. The \
              wire envelope itself stays JSON-line per the protocol \
              invariant — non-JSON formats land inside \
              `ResponseBody::Rendered { format, content }` so the \
              line is JSON but the payload is the formatted text. \
              v1 ships `json` (passthrough) and `yaml` (serde_yaml \
              over the same Serialize impls).",
    anchors: [
        crate::socket::OutputFormat,
        crate::socket::Request,
        crate::socket::ResponseBody,
        crate::socket::render_with_format,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  cast_stdlib::api::wire_format_negotiation::WireFormatNegotiation,
    concept: "output_format",
    why:     "The caller picks the wire encoding per request (`json` \
              vs `yaml`), orthogonal to which records are returned \
              (`dim`) and which subset matched (`select` predicate). \
              Adding a third format means a new renderer, not a new \
              endpoint.",
    tags:    ["cast_watch"],
}

cast::compare! {
    cast_emit_format @ cast::emit::Format: "cast-extract's CLI output \
        format. Variants `Text | Json | Markdown` chosen for rebuild \
        reports / PR comments / CI summaries — i.e. the *report* \
        shape (counts, summaries, parsed/error sections). One \
        `Report` value, three renderers; the loop in cast-extract's \
        main.rs picks one. Lives in `cast::emit::Format` and \
        operates on `cast::emit::model::Report`.",
    socket_output_format @ crate::socket::OutputFormat: "cast-watch's \
        wire format for query/walk responses. Variants `Json | Yaml` \
        chosen for *query* shape (concept dicts, anchor lists, \
        path tables) — different shape than cast-extract's reports, \
        so the variants and the render functions don't transfer \
        directly. Operates on `ResponseBody`. Adds `Yaml` because \
        scanning LLMs and humans both prefer YAML over JSON for \
        nested concept graphs.",
    tags: ["cast_watch"],
    note: "Two Format enums in the same crate-family is deliberate: \
           cast-extract's renders Reports for a CLI consumer; \
           cast-watch's renders ResponseBodies for socket consumers. \
           They share a precedent (one value → many renderers) but \
           not a type — same axis, different domains. If a future \
           consumer wants e.g. a markdown-shaped query response \
           (hits as a markdown table), it lands as a third variant \
           on `socket::OutputFormat`, not by reusing cast-extract's \
           Markdown renderer which expects a Report shape.",
}

/// Stage 4b: encode the typed body for the wire per `format`. v1
/// supports `Json` (passthrough) and `Yaml` (re-render via
/// `serde_yaml`). Anything else collapses into a `Rendered` body
/// whose `content` is the formatted string. The wire envelope stays
/// JSON-line — only the body content changes shape.
pub fn render_with_format(body: ResponseBody, format: OutputFormat) -> ResponseBody {
    match format {
        OutputFormat::Json => body,
        OutputFormat::Yaml => match serde_yaml::to_string(&body) {
            Ok(content) => ResponseBody::Rendered { format, content },
            Err(e) => ResponseBody::Error {
                message: format!("yaml render failed: {e}"),
            },
        },
    }
}

cast::concept! {
    name: "wire_format_renderer",
    summary: "Pure converter from a JSON-shaped ResponseBody into the \
              caller-requested wire format (yaml today; json is \
              identity).",
    anchors: [
        crate::socket::format::render_with_format,
    ],
    tags: ["cast_watch_socket"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "wire_format_renderer",
    why: "Output is a function of (body, format) alone.",
}

cast::continues_in! {
    target: cast_stdlib::integration::format_converter,
    concept: "wire_format_renderer",
    why: lazy,
}
