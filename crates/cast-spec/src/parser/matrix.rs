//! `cast::matrix!` — a header row plus N rows that share its arity.
//!
//! ```text
//! cast::matrix! {
//!     columns: [derived, stored, scope, example],
//!     rows: {
//!         hotp_tan        @ ...::HotpTan:        ["yes", "seed only", "any login", "RFC 4226"],
//!         static_tan_list @ ...::StaticTanList:  ["no",  "finite bag", "any login", "recovery codes"],
//!         transaction_tan @ ...::TransactionTan: ["yes", "server-side", "bound to txn", "photoTAN"],
//!     },
//! }
//! ```
//!
//! Per-row anchors are optional but warned. The parser enforces that
//! every row's value-list has exactly `columns.len()` entries — silent
//! column drift is the failure mode this macro exists to prevent.

use super::common::CommonFields;
use super::manual::FieldDoc;
use super::path_anchor::PathAnchor;
use super::take::{require, step_separator, take_ident_list};
use syn::{
    braced, bracketed, parse::Parse, parse::ParseStream, punctuated::Punctuated, Ident, LitStr,
    Token,
};

/// Schema-side mirror of the fields accepted by `Matrix::parse`.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "columns",
        kind: "list<ident>",
        required: true,
        doc: "Header row — column names. Every row's value list must have exactly this many entries.",
    },
    FieldDoc {
        name: "rows",
        kind: "block { name @ rust_path: [...], ... }",
        required: true,
        doc: "Each row anchors at a Rust item and carries `columns.len()` string values.",
    },
];

#[derive(Debug, Clone)]
pub struct Matrix {
    pub columns: Vec<Ident>,
    pub rows: Vec<MatrixRow>,
    pub common: CommonFields,
}

#[derive(Debug, Clone)]
pub struct MatrixRow {
    pub anchor: PathAnchor,
    pub values: Vec<String>,
}

impl Parse for MatrixRow {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let anchor: PathAnchor = input.parse()?;
        input.parse::<Token![:]>()?;
        let content;
        bracketed!(content in input);
        let punct: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated(&content)?;
        Ok(MatrixRow {
            anchor,
            values: punct.into_iter().map(|l| l.value()).collect(),
        })
    }
}

impl Parse for Matrix {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut columns: Option<Vec<Ident>> = None;
        let mut rows: Option<Vec<MatrixRow>> = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            match name.to_string().as_str() {
                "columns" => take_ident_list(&name, input, &mut columns)?,
                "rows" => {
                    if rows.is_some() {
                        return Err(syn::Error::new(name.span(), "duplicate field: rows"));
                    }
                    input.parse::<Token![:]>()?;
                    let content;
                    braced!(content in input);
                    let punct: Punctuated<MatrixRow, Token![,]> =
                        Punctuated::parse_terminated(&content)?;
                    rows = Some(punct.into_iter().collect());
                }
                _ => {
                    if !common.take_field_if_known(&name, input)? {
                        return Err(syn::Error::new(
                            name.span(),
                            format!("unknown field for cast::matrix!: {name}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        let columns = require(columns, "columns")?;
        let rows = require(rows, "rows")?;
        if columns.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "cast::matrix!: `columns` must contain at least one entry",
            ));
        }
        if rows.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "cast::matrix!: `rows` must contain at least one entry",
            ));
        }
        for row in &rows {
            if row.values.len() != columns.len() {
                return Err(syn::Error::new(
                    row.anchor.name.span(),
                    format!(
                        "matrix row `{}` has {} values but `columns` has {} entries",
                        row.anchor.name,
                        row.values.len(),
                        columns.len()
                    ),
                ));
            }
        }

        Ok(Matrix {
            columns,
            rows,
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
    fn parses_full_matrix() {
        let src = indoc! {r#"
            columns: [a, b, c],
            rows: {
                row1 @ M::Row1: ["1", "2", "3"],
                row2 @ M::Row2: ["x", "y", "z"],
            }
        "#};
        let parsed: Matrix = parse_str(src).expect("parse");
        assert_eq!(parsed.columns.len(), 3);
        assert_eq!(parsed.rows.len(), 2);
    }

    #[test]
    fn rejects_arity_mismatch() {
        let src = indoc! {r#"
            columns: [a, b, c],
            rows: { r @ M::r: ["only", "two"] }
        "#};
        let err = parse_str::<Matrix>(src).unwrap_err();
        assert!(err.to_string().contains("has 2 values but `columns` has 3"));
    }

    #[test]
    fn allows_unanchored_row() {
        let src = indoc! {r#"
            columns: [a],
            rows: { plain: ["v"] }
        "#};
        let parsed: Matrix = parse_str(src).expect("parse");
        assert!(parsed.rows[0].anchor.path.is_none());
    }
}
