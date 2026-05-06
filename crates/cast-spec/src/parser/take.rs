//! Field-extraction helpers shared by every macro parser.
//!
//! Each macro body is structurally `field: value, field: value, ...`,
//! so the per-macro `Parse` impls boil down to "match name → take_*
//! helper → store in slot". Centralising the helpers keeps every parser
//! short and gives one place to fix duplicate-field detection or value
//! parsing if the grammar evolves.

use super::common::WhyValue;
use proc_macro2::Span;
use syn::{
    bracketed, parse::ParseStream, punctuated::Punctuated, Ident, LitStr, Path, Token,
};

/// Consume `: "string"` and store. Errors if the slot is already filled
/// (duplicate field) or the value isn't a string literal.
pub fn take_str(name: &Ident, input: ParseStream, slot: &mut Option<String>) -> syn::Result<()> {
    duplicate_check(name, slot)?;
    input.parse::<Token![:]>()?;
    let lit: LitStr = input.parse()?;
    *slot = Some(lit.value());
    Ok(())
}

/// Consume `: a::b::c` and store.
pub fn take_path(name: &Ident, input: ParseStream, slot: &mut Option<Path>) -> syn::Result<()> {
    duplicate_check(name, slot)?;
    input.parse::<Token![:]>()?;
    let path: Path = input.parse()?;
    *slot = Some(path);
    Ok(())
}

/// Consume `: identifier` and store.
pub fn take_ident(name: &Ident, input: ParseStream, slot: &mut Option<Ident>) -> syn::Result<()> {
    duplicate_check(name, slot)?;
    input.parse::<Token![:]>()?;
    let ident: Ident = input.parse()?;
    *slot = Some(ident);
    Ok(())
}

/// Consume `: [a::b, c::d, ...]` and store. The bracketed list may have
/// a trailing comma. Empty lists are allowed at this layer — per-macro
/// rules enforce minimum lengths.
pub fn take_path_list(
    name: &Ident,
    input: ParseStream,
    slot: &mut Option<Vec<Path>>,
) -> syn::Result<()> {
    duplicate_check(name, slot)?;
    input.parse::<Token![:]>()?;
    let content;
    bracketed!(content in input);
    let punct: Punctuated<Path, Token![,]> = Punctuated::parse_terminated(&content)?;
    *slot = Some(punct.into_iter().collect());
    Ok(())
}

/// Consume `: [foo, bar, ...]` and store.
pub fn take_ident_list(
    name: &Ident,
    input: ParseStream,
    slot: &mut Option<Vec<Ident>>,
) -> syn::Result<()> {
    duplicate_check(name, slot)?;
    input.parse::<Token![:]>()?;
    let content;
    bracketed!(content in input);
    let punct: Punctuated<Ident, Token![,]> = Punctuated::parse_terminated(&content)?;
    *slot = Some(punct.into_iter().collect());
    Ok(())
}

/// Consume `: ["a", "b", ...]` and store.
pub fn take_str_list(
    name: &Ident,
    input: ParseStream,
    slot: &mut Option<Vec<String>>,
) -> syn::Result<()> {
    duplicate_check(name, slot)?;
    input.parse::<Token![:]>()?;
    let content;
    bracketed!(content in input);
    let punct: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated(&content)?;
    *slot = Some(punct.into_iter().map(|l| l.value()).collect());
    Ok(())
}

/// Consume `: "string"` OR `: lazy` and store. Used for the
/// `why:` field on `continues_in!` / `io::continues_in!` so the writer
/// can mark a deferred-explanation edge with the bare ident `lazy`
/// instead of typing prose during a fast Layer-0 → top sweep.
pub fn take_why(name: &Ident, input: ParseStream, slot: &mut Option<WhyValue>) -> syn::Result<()> {
    duplicate_check(name, slot)?;
    input.parse::<Token![:]>()?;
    if input.peek(LitStr) {
        let lit: LitStr = input.parse()?;
        *slot = Some(WhyValue::Prose { text: lit.value() });
        Ok(())
    } else if input.peek(Ident) {
        let ident: Ident = input.parse()?;
        if ident == "lazy" {
            *slot = Some(WhyValue::Lazy);
            Ok(())
        } else {
            Err(syn::Error::new(
                ident.span(),
                format!("expected string literal or `lazy`, got `{ident}`"),
            ))
        }
    } else {
        Err(syn::Error::new(
            input.span(),
            "expected string literal or `lazy`",
        ))
    }
}

/// Pull a required value out of an `Option`, producing a `missing
/// required field` error when absent.
pub fn require<T>(slot: Option<T>, field: &str) -> syn::Result<T> {
    slot.ok_or_else(|| syn::Error::new(Span::call_site(), format!("missing required field: {field}")))
}

/// Step the parser past a trailing comma, or return false to signal
/// "this was the last field". Saves the same `if peek { parse } else
/// { break }` clause from appearing in every macro parser.
pub fn step_separator(input: ParseStream) -> syn::Result<bool> {
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        Ok(!input.is_empty())
    } else {
        Ok(false)
    }
}

fn duplicate_check<T>(name: &Ident, slot: &Option<T>) -> syn::Result<()> {
    if slot.is_some() {
        return Err(syn::Error::new(
            name.span(),
            format!("duplicate field: {name}"),
        ));
    }
    Ok(())
}

cast::concept! {
    name: "macro_field_taker",
    summary: "Field-extraction helpers shared by every macro parser. Each \
              consumes one piece of structured syn input (`: \"string\"`, \
              `: a::b::c`, `: [...]`) and writes it into a caller-owned \
              Option slot, erroring on duplicate-field. Deterministic — \
              same input always produces the same slot value or the same \
              syn::Error — but not pure: the slot mutation and the \
              ParseStream advance are observable side effects.",
    anchors: [
        crate::parser::take::take_str,
        crate::parser::take::take_path,
        crate::parser::take::take_ident,
        crate::parser::take::take_path_list,
        crate::parser::take::take_ident_list,
        crate::parser::take::take_str_list,
        crate::parser::take::step_separator,
    ],
    tags: ["cast_spec_parser", "field_extraction"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "macro_field_taker",
    why: "Same syn input + same starting slot state always yields the \
          same final slot state and the same Result. Variation comes only \
          from the inputs.",
}

cast::concept! {
    name: "macro_field_finalizer",
    summary: "Pure helpers that close out a macro-body parse: `require` \
              unwraps a required Option<T> into Result<T, syn::Error> with \
              a uniform 'missing required field' message; `duplicate_check` \
              fails when a slot has already been written. Both are pure — \
              given the same Option and the same field name, they always \
              return the same Result, with no observable side effects.",
    anchors: [
        crate::parser::take::require,
        crate::parser::take::duplicate_check,
    ],
    tags: ["cast_spec_parser", "field_extraction"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "macro_field_finalizer",
    why: "Read-only on the inputs; output is a function of (Option<T>, &str) \
          alone; no side effects.",
}
