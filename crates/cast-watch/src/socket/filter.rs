use super::protocol::{QueryPredicate, WhyKindFilter};
use cast::parser::common::WhyValue;

pub fn invocation_matches(
    inv: &cast::emit::model::InvocationReport,
    filter: Option<&QueryPredicate>,
) -> bool {
    let Some(f) = filter else {
        return true;
    };
    if let Some(tag) = &f.tag {
        if !inv.tags.iter().any(|t| t == tag) {
            return false;
        }
    }
    if let Some(tag_not) = &f.tag_not {
        if inv.tags.iter().any(|t| t == tag_not) {
            return false;
        }
    }
    if let Some(sub) = &f.file_contains {
        if !inv.file.contains(sub) {
            return false;
        }
    }
    if let Some(kind) = &f.kind {
        if inv.kind != *kind {
            return false;
        }
    }
    true
}

pub fn concept_matches(
    c: &cast::emit::model::ConceptReport,
    filter: Option<&QueryPredicate>,
) -> bool {
    let Some(f) = filter else {
        return true;
    };
    if let Some(sub) = &f.file_contains {
        if !c.declarations.iter().any(|d| d.file.contains(sub)) {
            return false;
        }
    }
    if let Some(anchor) = &f.has_anchor {
        if !c
            .declarations
            .iter()
            .any(|d| d.anchors.iter().any(|a| &a.path == anchor))
        {
            return false;
        }
    }
    if let Some(role) = f.role {
        if !c
            .declarations
            .iter()
            .any(|d| d.anchors.iter().any(|a| a.role == role))
        {
            return false;
        }
    }
    // Tag and tag_not check declarations, since a concept's tags are
    // sourced from its `cast::concept!` block(s). A concept matches
    // `tag: X` if ANY declaration carries tag X (in its tags list),
    // and matches `tag_not: X` if NO declaration carries tag X.
    if let Some(tag) = &f.tag {
        if !c
            .declarations
            .iter()
            .any(|d| d.tags.iter().any(|t| t == tag))
        {
            return false;
        }
    }
    if let Some(tag_not) = &f.tag_not {
        if c.declarations
            .iter()
            .any(|d| d.tags.iter().any(|t| t == tag_not))
        {
            return false;
        }
    }
    if let Some(wk) = f.why_kind {
        if !c.edges.iter().any(|e| edge_why_matches(&e.why, wk)) {
            return false;
        }
    }
    true
}

/// Does this edge's `why` match the requested `WhyKindFilter`?
/// `Lazy` matches `Some(WhyValue::Lazy)`, `Prose` matches
/// `Some(WhyValue::Prose { .. })`, `Absent` matches `None`.
fn edge_why_matches(why: &Option<WhyValue>, kind: WhyKindFilter) -> bool {
    match (why, kind) {
        (Some(WhyValue::Lazy), WhyKindFilter::Lazy) => true,
        (Some(WhyValue::Prose { .. }), WhyKindFilter::Prose) => true,
        (None, WhyKindFilter::Absent) => true,
        _ => false,
    }
}

pub fn path_matches(
    inv: &cast::emit::model::InvocationReport,
    p: &cast::emit::model::PathReport,
    filter: Option<&QueryPredicate>,
) -> bool {
    let Some(f) = filter else {
        return true;
    };
    if let Some(role) = f.role {
        if p.role != Some(role) {
            return false;
        }
    }
    if let Some(tag) = &f.tag {
        if !inv.tags.iter().any(|t| t == tag) {
            return false;
        }
    }
    if let Some(tag_not) = &f.tag_not {
        if inv.tags.iter().any(|t| t == tag_not) {
            return false;
        }
    }
    if let Some(sub) = &f.file_contains {
        if !inv.file.contains(sub) {
            return false;
        }
    }
    if let Some(kind) = &f.kind {
        if inv.kind != *kind {
            return false;
        }
    }
    if let Some(outcome) = &f.outcome {
        if p.outcome != *outcome {
            return false;
        }
    }
    true
}

cast::concept! {
    name: "query_predicate_matchers",
    summary: "Pure boolean filter helpers used by handle_query_select. \
              Take a record + an optional QueryPredicate; return \
              whether the record passes.",
    anchors: [
        crate::socket::filter::invocation_matches,
        crate::socket::filter::concept_matches,
        crate::socket::filter::path_matches,
        crate::socket::filter::edge_why_matches,
    ],
    tags: ["cast_watch_socket"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "query_predicate_matchers",
    why: "Boolean predicates over (record, predicate); no I/O, no \
          mutation.",
}
