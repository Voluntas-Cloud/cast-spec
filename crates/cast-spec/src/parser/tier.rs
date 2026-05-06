//! `cast::tier!` — ordered named buckets along one axis (stability,
//! risk, maturity, …).
//!
//! ```text
//! cast::tier! {
//!     axis:      stability,
//!     direction: increasing,
//!     of:        sample::stability::StabilityLevel,
//!     stages: {
//!         fragile          @ ...::Fragile:           "single point of failure",
//!         recoverable      @ ...::Recoverable:       "downtime, data survives",
//!         resilient        @ ...::Resilient:         "degraded service",
//!         highly_available @ ...::HighlyAvailable:   "transparent",
//!     },
//! }
//! ```
//!
//! `direction` defaults to `increasing` when absent (see GRAMMAR.md).
//! Per-stage `@ path` anchors are optional but warned-on-missing.

use super::common::CommonFields;
use super::manual::FieldDoc;
use super::path_anchor::PathAnchor;
use super::take::{require, step_separator, take_ident, take_path};
use syn::{
    braced, parse::Parse, parse::ParseStream, punctuated::Punctuated, Ident, LitStr, Path,
    Token,
};

/// Schema-side mirror of the fields accepted by `Tier::parse`.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "axis",
        kind: "ident",
        required: true,
        doc: "Name of the dimension being tiered.",
    },
    FieldDoc {
        name: "direction",
        kind: "ident",
        required: false,
        doc: "increasing | decreasing — direction of progression along the axis. Defaults to increasing.",
    },
    FieldDoc {
        name: "of",
        kind: "rust_path",
        required: false,
        doc: "Type whose variants are being tiered (e.g. an enum).",
    },
    FieldDoc {
        name: "stages",
        kind: "block { name @ rust_path: \"description\", ... }",
        required: true,
        doc: "Named tiers; per-stage anchor optional.",
    },
];

#[derive(Debug, Clone)]
pub struct Tier {
    pub axis: Ident,
    pub direction: Direction,
    pub of: Option<Path>,
    pub stages: Vec<TierStage>,
    pub common: CommonFields,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Increasing,
    Decreasing,
}

#[derive(Debug, Clone)]
pub struct TierStage {
    pub anchor: PathAnchor,
    pub description: String,
}

impl Parse for TierStage {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let anchor: PathAnchor = input.parse()?;
        input.parse::<Token![:]>()?;
        let desc: LitStr = input.parse()?;
        Ok(TierStage {
            anchor,
            description: desc.value(),
        })
    }
}

impl Parse for Tier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut axis: Option<Ident> = None;
        let mut direction_ident: Option<Ident> = None;
        let mut of: Option<Path> = None;
        let mut stages: Option<Vec<TierStage>> = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            match name.to_string().as_str() {
                "axis" => take_ident(&name, input, &mut axis)?,
                "direction" => take_ident(&name, input, &mut direction_ident)?,
                "of" => take_path(&name, input, &mut of)?,
                "stages" => {
                    if stages.is_some() {
                        return Err(syn::Error::new(name.span(), "duplicate field: stages"));
                    }
                    input.parse::<Token![:]>()?;
                    let content;
                    braced!(content in input);
                    let punct: Punctuated<TierStage, Token![,]> =
                        Punctuated::parse_terminated(&content)?;
                    stages = Some(punct.into_iter().collect());
                }
                _ => {
                    if !common.take_field_if_known(&name, input)? {
                        return Err(syn::Error::new(
                            name.span(),
                            format!("unknown field for cast::tier!: {name}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        let direction = match direction_ident {
            None => Direction::Increasing,
            Some(id) => match id.to_string().as_str() {
                "increasing" => Direction::Increasing,
                "decreasing" => Direction::Decreasing,
                other => {
                    return Err(syn::Error::new(
                        id.span(),
                        format!("direction must be `increasing` or `decreasing`, got `{other}`"),
                    ));
                }
            },
        };

        let stages = require(stages, "stages")?;
        if stages.len() < 2 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "cast::tier!: `stages` requires at least 2 entries",
            ));
        }

        Ok(Tier {
            axis: require(axis, "axis")?,
            direction,
            of,
            stages,
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
    fn parses_full_tier() {
        let src = indoc! {r#"
            axis:      stability,
            direction: increasing,
            of:        s::S,
            stages: {
                a @ s::S::A: "low",
                b @ s::S::B: "high",
            }
        "#};
        let parsed: Tier = parse_str(src).expect("parse");
        assert_eq!(parsed.axis.to_string(), "stability");
        assert_eq!(parsed.direction, Direction::Increasing);
        assert!(parsed.of.is_some());
        assert_eq!(parsed.stages.len(), 2);
    }

    #[test]
    fn defaults_direction_to_increasing() {
        let src = indoc! {r#"
            axis: risk,
            stages: {
                low  @ R::Low:  "ok",
                high @ R::High: "bad",
            }
        "#};
        let parsed: Tier = parse_str(src).expect("parse");
        assert_eq!(parsed.direction, Direction::Increasing);
    }

    #[test]
    fn rejects_unknown_direction() {
        let src = indoc! {r#"
            axis: x,
            direction: sideways,
            stages: { a @ A::a: "...", b @ A::b: "..." }
        "#};
        let err = parse_str::<Tier>(src).unwrap_err();
        assert!(err.to_string().contains("direction must be"));
    }
}
