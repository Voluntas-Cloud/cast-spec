//! `cast::pipeline!` — a directed chain of named stages, each anchored
//! to a Rust path.
//!
//! ```text
//! cast::pipeline! {
//!     stages: {
//!         raw_facts               @ sample_agent::heartbeat::report,
//!         normalized_capabilities @ sample::stability::normalize,
//!         ...
//!     },
//!     flow: [
//!         raw_facts               -> normalized_capabilities,
//!         normalized_capabilities -> cluster_capabilities,
//!         ...
//!     ],
//!     cyclic: false,
//! }
//! ```
//!
//! Stages are *named* and declared once; `flow` references them by name
//! so anchor paths aren't repeated on every edge. Per-stage path
//! anchors are recommended-but-warned (see GRAMMAR.md), so this parser
//! accepts stages without `@ path` and surfaces the per-stage Option.

use super::common::CommonFields;
use super::manual::FieldDoc;
use super::path_anchor::PathAnchor;
use super::take::{require, step_separator, take_ident};
use syn::{
    braced, bracketed, parse::Parse, parse::ParseStream, punctuated::Punctuated, Ident,
    LitBool, Token,
};

/// Schema-side mirror of the fields accepted by `Pipeline::parse`.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "stages",
        kind: "block { name @ rust_path, ... }",
        required: true,
        doc: "Named stages, each anchored at a Rust item. Per-stage `@ path` is recommended but optional.",
    },
    FieldDoc {
        name: "flow",
        kind: "list<edge>",
        required: false,
        doc: "Directed edges between stage names: `a -> b`.",
    },
    FieldDoc {
        name: "cyclic",
        kind: "bool",
        required: false,
        doc: "Whether the flow graph is allowed to contain cycles. Defaults to false.",
    },
    FieldDoc {
        name: "entry",
        kind: "ident",
        required: false,
        doc: "Optional name of the stage that's the pipeline's entry point.",
    },
];

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub stages: Vec<PathAnchor>,
    pub flow: Vec<FlowEdge>,
    pub cyclic: bool,
    pub entry: Option<Ident>,
    pub common: CommonFields,
}

#[derive(Debug, Clone)]
pub struct FlowEdge {
    pub from: Ident,
    pub to: Ident,
}

impl Parse for FlowEdge {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let from: Ident = input.parse()?;
        input.parse::<Token![->]>()?;
        let to: Ident = input.parse()?;
        Ok(FlowEdge { from, to })
    }
}

impl Parse for Pipeline {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut stages: Option<Vec<PathAnchor>> = None;
        let mut flow: Option<Vec<FlowEdge>> = None;
        let mut cyclic: Option<bool> = None;
        let mut entry: Option<Ident> = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            match name.to_string().as_str() {
                "stages" => {
                    if stages.is_some() {
                        return Err(syn::Error::new(name.span(), "duplicate field: stages"));
                    }
                    input.parse::<Token![:]>()?;
                    let content;
                    braced!(content in input);
                    let punct: Punctuated<PathAnchor, Token![,]> =
                        Punctuated::parse_terminated(&content)?;
                    stages = Some(punct.into_iter().collect());
                }
                "flow" => {
                    if flow.is_some() {
                        return Err(syn::Error::new(name.span(), "duplicate field: flow"));
                    }
                    input.parse::<Token![:]>()?;
                    let content;
                    bracketed!(content in input);
                    let punct: Punctuated<FlowEdge, Token![,]> =
                        Punctuated::parse_terminated(&content)?;
                    flow = Some(punct.into_iter().collect());
                }
                "cyclic" => {
                    if cyclic.is_some() {
                        return Err(syn::Error::new(name.span(), "duplicate field: cyclic"));
                    }
                    input.parse::<Token![:]>()?;
                    let lit: LitBool = input.parse()?;
                    cyclic = Some(lit.value);
                }
                "entry" => take_ident(&name, input, &mut entry)?,
                _ => {
                    if !common.take_field_if_known(&name, input)? {
                        return Err(syn::Error::new(
                            name.span(),
                            format!("unknown field for cast::pipeline!: {name}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        let stages = require(stages, "stages")?;
        let flow = require(flow, "flow")?;
        if stages.len() < 2 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "cast::pipeline!: `stages` requires at least 2 entries",
            ));
        }
        if flow.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "cast::pipeline!: `flow` requires at least 1 edge",
            ));
        }
        // Every flow ident must appear in stages — no implicit nodes.
        let stage_names: std::collections::HashSet<String> =
            stages.iter().map(|s| s.name.to_string()).collect();
        for edge in &flow {
            for end in [&edge.from, &edge.to] {
                if !stage_names.contains(&end.to_string()) {
                    return Err(syn::Error::new(
                        end.span(),
                        format!("flow edge references unknown stage `{end}`"),
                    ));
                }
            }
        }

        Ok(Pipeline {
            stages,
            flow,
            cyclic: cyclic.unwrap_or(false),
            entry,
            common,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use syn::parse_str;

    #[test]
    fn parses_full_pipeline() {
        let src = indoc! {r#"
            stages: {
                a @ foo::a,
                b @ foo::b,
                c @ foo::c,
            },
            flow: [
                a -> b,
                b -> c,
            ]
        "#};
        let parsed: Pipeline = parse_str(src).expect("parse");
        assert_eq!(parsed.stages.len(), 3);
        assert_eq!(parsed.flow.len(), 2);
        assert!(!parsed.cyclic);
    }

    #[test]
    fn unanchored_stage_is_allowed() {
        // Per GRAMMAR.md, per-stage anchors are warn-only, not error.
        let src = indoc! {r#"
            stages: { a, b @ foo::b },
            flow:   [a -> b]
        "#};
        let parsed: Pipeline = parse_str(src).expect("parse");
        assert!(parsed.stages[0].path.is_none());
        assert!(parsed.stages[1].path.is_some());
    }

    #[test]
    fn rejects_flow_referencing_unknown_stage() {
        let src = indoc! {r#"
            stages: { a @ foo::a, b @ foo::b },
            flow:   [a -> c]
        "#};
        let err = parse_str::<Pipeline>(src).unwrap_err();
        assert!(err.to_string().contains("unknown stage `c`"));
    }

    #[test]
    fn cyclic_flag_round_trips() {
        let src = indoc! {r#"
            stages: { a @ foo::a, b @ foo::b },
            flow:   [a -> b, b -> a],
            cyclic: true
        "#};
        let parsed: Pipeline = parse_str(src).expect("parse");
        assert!(parsed.cyclic);
    }
}
