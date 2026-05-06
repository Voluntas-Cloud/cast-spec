//! The `Ident @ Path` shape used by entry-level path anchors in
//! `compare!`, `pipeline!`, `tier!`, `matrix!`.
//!
//! Example surface:
//!
//! ```text
//! intent_id @ sample::types::IntentId
//! ```

use syn::{parse::Parse, parse::ParseStream, Ident, Path, Token};

#[derive(Debug, Clone)]
pub struct PathAnchor {
    pub name: Ident,
    pub path: Option<Path>,
}

impl Parse for PathAnchor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let path = if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Some(input.parse::<Path>()?)
        } else {
            None
        };
        Ok(Self { name, path })
    }
}
