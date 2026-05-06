//! `cast::policy!` — workspace-level convention declaration.
//!
//! Lives in `Cast.cast` (or any other `.cast` file) at the repo root.
//! Declares which spec layouts the repo wants the analyzer to enforce
//! (warn about) when downstream commits enforcement land.
//!
//! ```text
//! cast::policy! {
//!     layout: sidecar_only,
//!     inline_in_rust: forbid,
//!     umbrella_files: ["Cast.cast"],
//!     note: "Specs live in *.rs.cast siblings; Cast.cast is the only umbrella.",
//! }
//! ```
//!
//! All fields optional. With no `cast::policy!` block in the repo, the
//! analyzer behaves as today (mixed layout, all shapes allowed).
//!
//! Enforcement is NOT implemented in this commit; the parser stores the
//! parsed value and the manual lists the macro. A follow-up commit
//! wires warnings off the stored value.

use super::common::CommonFields;
use super::manual::FieldDoc;
use super::take::{step_separator, take_ident, take_str_list};
use syn::{parse::Parse, parse::ParseStream, Ident};

/// Schema-side mirror of the fields accepted by `Policy::parse`.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "layout",
        kind: "ident",
        required: false,
        doc: "Primary spec layout for the repo. Recognised values: \
              `sidecar_only` (specs live in *.rs.cast siblings; no inline), \
              `sidecar_preferred` (sidecars preferred but inline tolerated \
              with a warning), `inline_only` (inline-only; no .cast files), \
              `mixed` (default; anything goes).",
    },
    FieldDoc {
        name: "inline_in_rust",
        kind: "ident",
        required: false,
        doc: "Stance toward `cast::*!` blocks inside `.rs` files. \
              Recognised values: `allow` (default; no warning), `warn` \
              (emit a soft warning per inline block), `forbid` (treat as \
              policy violation in reports).",
    },
    FieldDoc {
        name: "umbrella_files",
        kind: "list<string>",
        required: false,
        doc: "Allowed list of umbrella `.cast` filenames. When set, any \
              non-sidecar `.cast` file whose filename isn't in this list \
              triggers an `unexpected_umbrella` warning. Empty/unset = no \
              restriction.",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPolicy {
    SidecarOnly,
    SidecarPreferred,
    InlineOnly,
    Mixed,
}

impl LayoutPolicy {
    /// Wire-shape token. Mirrors the source identifier accepted by the
    /// parser; emit/model.rs uses this for the JSON-serialised
    /// `PolicyDecl.layout` field.
    pub fn as_str(self) -> &'static str {
        match self {
            LayoutPolicy::SidecarOnly => "sidecar_only",
            LayoutPolicy::SidecarPreferred => "sidecar_preferred",
            LayoutPolicy::InlineOnly => "inline_only",
            LayoutPolicy::Mixed => "mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineInRustPolicy {
    Allow,
    Warn,
    Forbid,
}

impl InlineInRustPolicy {
    /// Wire-shape token. Mirrors the source identifier accepted by the
    /// parser.
    pub fn as_str(self) -> &'static str {
        match self {
            InlineInRustPolicy::Allow => "allow",
            InlineInRustPolicy::Warn => "warn",
            InlineInRustPolicy::Forbid => "forbid",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub layout: Option<LayoutPolicy>,
    pub inline_in_rust: Option<InlineInRustPolicy>,
    pub umbrella_files: Vec<String>,
    pub common: CommonFields,
}

impl Parse for Policy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut layout_ident: Option<Ident> = None;
        let mut inline_ident: Option<Ident> = None;
        let mut umbrella_files: Option<Vec<String>> = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            match name.to_string().as_str() {
                "layout" => take_ident(&name, input, &mut layout_ident)?,
                "inline_in_rust" => take_ident(&name, input, &mut inline_ident)?,
                "umbrella_files" => take_str_list(&name, input, &mut umbrella_files)?,
                _ => {
                    if !common.take_field_if_known(&name, input)? {
                        return Err(syn::Error::new(
                            name.span(),
                            format!("unknown field for cast::policy!: {name}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        let layout = layout_ident
            .map(|ident| match ident.to_string().as_str() {
                "sidecar_only" => Ok(LayoutPolicy::SidecarOnly),
                "sidecar_preferred" => Ok(LayoutPolicy::SidecarPreferred),
                "inline_only" => Ok(LayoutPolicy::InlineOnly),
                "mixed" => Ok(LayoutPolicy::Mixed),
                other => Err(syn::Error::new(
                    ident.span(),
                    format!(
                        "unknown layout value `{other}`; expected one of: \
                         sidecar_only, sidecar_preferred, inline_only, mixed"
                    ),
                )),
            })
            .transpose()?;

        let inline_in_rust = inline_ident
            .map(|ident| match ident.to_string().as_str() {
                "allow" => Ok(InlineInRustPolicy::Allow),
                "warn" => Ok(InlineInRustPolicy::Warn),
                "forbid" => Ok(InlineInRustPolicy::Forbid),
                other => Err(syn::Error::new(
                    ident.span(),
                    format!(
                        "unknown inline_in_rust value `{other}`; expected one of: \
                         allow, warn, forbid"
                    ),
                )),
            })
            .transpose()?;

        Ok(Policy {
            layout,
            inline_in_rust,
            umbrella_files: umbrella_files.unwrap_or_default(),
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
    fn parses_full_policy() {
        let src = indoc! {r#"
            layout: sidecar_only,
            inline_in_rust: forbid,
            umbrella_files: ["Cast.cast"],
            tags: ["t"],
            since: "v0.1",
            note: "Specs live in *.rs.cast siblings; Cast.cast is the only umbrella."
        "#};
        let parsed: Policy = parse_str(src).expect("parse");
        assert_eq!(parsed.layout, Some(LayoutPolicy::SidecarOnly));
        assert_eq!(parsed.inline_in_rust, Some(InlineInRustPolicy::Forbid));
        assert_eq!(parsed.umbrella_files, vec!["Cast.cast"]);
        assert_eq!(parsed.common.tags, vec!["t"]);
        assert_eq!(parsed.common.since.as_deref(), Some("v0.1"));
        assert!(parsed.common.note.is_some());
    }

    #[test]
    fn empty_policy_is_legal() {
        let parsed: Policy = parse_str("").expect("parse");
        assert!(parsed.layout.is_none());
        assert!(parsed.inline_in_rust.is_none());
        assert!(parsed.umbrella_files.is_empty());
    }

    #[test]
    fn rejects_unknown_layout_value() {
        let src = "layout: brand_new_thing";
        let err = parse_str::<Policy>(src).unwrap_err();
        assert!(err.to_string().contains("unknown layout value"));
    }

    #[test]
    fn rejects_unknown_inline_value() {
        let src = "inline_in_rust: maybe";
        let err = parse_str::<Policy>(src).unwrap_err();
        assert!(err.to_string().contains("unknown inline_in_rust value"));
    }

    #[test]
    fn rejects_unknown_field() {
        let src = r#"frobnicate: "12""#;
        let err = parse_str::<Policy>(src).unwrap_err();
        assert!(err.to_string().contains("unknown field for cast::policy"));
    }

    #[test]
    fn rejects_duplicate_layout() {
        let src = "layout: sidecar_only, layout: mixed";
        let err = parse_str::<Policy>(src).unwrap_err();
        assert!(err.to_string().contains("duplicate field: layout"));
    }
}
