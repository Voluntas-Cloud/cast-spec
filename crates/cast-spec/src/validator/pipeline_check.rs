//! Pipeline semantic checks — beyond shape, into structure.
//!
//! The parser already enforces `stages.len() >= 2`, `flow.len() >= 1`,
//! and "every flow endpoint names a declared stage." This module checks
//! the rest:
//!
//! - **UnpermittedCycle**: a cycle exists in `flow` but the pipeline
//!   was not declared `cyclic: true`. Cycles aren't pathological per
//!   se (retry loops, refresh cycles), but they have to be acknowledged.
//! - **UnconnectedStage**: a stage declared in `stages` that no flow
//!   edge touches. Almost always a typo or a leftover from refactoring.
//! - **DisconnectedComponents**: the flow graph has more than one
//!   weakly-connected component. Two parallel sub-pipelines stuffed
//!   into one declaration; the user probably wanted two `pipeline!`
//!   invocations.
//!
//! All three are reported as errors (CI-failing). The grammar's softer
//! "warning" category is reserved for graph-level concerns where
//! asymmetry is sometimes correct; here it isn't.

use crate::parser::Pipeline;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PipelineDiagnostic {
    pub kind: PipelineDiagKind,
}

#[derive(Debug, Clone)]
pub enum PipelineDiagKind {
    /// Cycle present, `cyclic: true` not set. `cycle` is the closed
    /// path: `a -> b -> ... -> a`.
    UnpermittedCycle { cycle: Vec<String> },
    /// Stage declared in `stages:` but no flow edge touches it.
    UnconnectedStage { stage: String },
    /// More than one weakly-connected component in the flow graph.
    DisconnectedComponents { components: Vec<Vec<String>> },
}

impl std::fmt::Display for PipelineDiagKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineDiagKind::UnpermittedCycle { cycle } => write!(
                f,
                "cycle without `cyclic: true`: {}",
                cycle.join(" -> ")
            ),
            PipelineDiagKind::UnconnectedStage { stage } => write!(
                f,
                "stage `{stage}` declared but not in any flow edge"
            ),
            PipelineDiagKind::DisconnectedComponents { components } => {
                let rendered: Vec<String> = components
                    .iter()
                    .map(|c| format!("[{}]", c.join(", ")))
                    .collect();
                write!(
                    f,
                    "pipeline splits into {} disconnected components: {}",
                    components.len(),
                    rendered.join(", ")
                )
            }
        }
    }
}

pub fn validate_pipeline(p: &Pipeline) -> Vec<PipelineDiagnostic> {
    let mut out = Vec::new();
    let stages: Vec<String> = p.stages.iter().map(|s| s.name.to_string()).collect();

    let mut directed: HashMap<String, Vec<String>> = HashMap::new();
    let mut undirected: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_flow: HashSet<String> = HashSet::new();
    for s in &stages {
        directed.entry(s.clone()).or_default();
        undirected.entry(s.clone()).or_default();
    }
    for edge in &p.flow {
        let from = edge.from.to_string();
        let to = edge.to.to_string();
        directed.get_mut(&from).unwrap().push(to.clone());
        undirected.get_mut(&from).unwrap().push(to.clone());
        undirected.get_mut(&to).unwrap().push(from.clone());
        in_flow.insert(from);
        in_flow.insert(to);
    }

    for s in &stages {
        if !in_flow.contains(s) {
            out.push(PipelineDiagnostic {
                kind: PipelineDiagKind::UnconnectedStage { stage: s.clone() },
            });
        }
    }

    if !p.cyclic {
        if let Some(cycle) = find_cycle(&stages, &directed) {
            out.push(PipelineDiagnostic {
                kind: PipelineDiagKind::UnpermittedCycle { cycle },
            });
        }
    }

    // Components are computed only over stages that participate in flow.
    // An "unconnected stage" is its own singleton component; reporting
    // it under both labels would be noise.
    let participating: Vec<String> = stages.iter().filter(|s| in_flow.contains(*s)).cloned().collect();
    let components = weakly_connected(&participating, &undirected);
    if components.len() > 1 {
        out.push(PipelineDiagnostic {
            kind: PipelineDiagKind::DisconnectedComponents { components },
        });
    }

    out
}

/// White=0, Gray=1, Black=2 iterative DFS. On a back-edge to a gray
/// node, reconstruct the cycle via the parent chain.
fn find_cycle(
    stages: &[String],
    directed: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    let mut color: HashMap<String, u8> = HashMap::new();
    let mut parent: HashMap<String, String> = HashMap::new();
    for start in stages {
        if color.get(start).copied().unwrap_or(0) != 0 {
            continue;
        }
        if let Some(c) = dfs_cycle(start, directed, &mut color, &mut parent) {
            return Some(c);
        }
    }
    None
}

fn dfs_cycle(
    start: &str,
    directed: &HashMap<String, Vec<String>>,
    color: &mut HashMap<String, u8>,
    parent: &mut HashMap<String, String>,
) -> Option<Vec<String>> {
    // Recursive DFS; pipelines are small (typically <20 stages).
    color.insert(start.to_string(), 1);
    if let Some(neighbors) = directed.get(start) {
        for next in neighbors {
            match color.get(next).copied().unwrap_or(0) {
                0 => {
                    parent.insert(next.clone(), start.to_string());
                    if let Some(c) = dfs_cycle(next, directed, color, parent) {
                        return Some(c);
                    }
                }
                1 => {
                    // Back-edge: walk parent chain from `start` up to `next`.
                    let mut cycle = vec![next.to_string()];
                    let mut cur = start.to_string();
                    cycle.push(cur.clone());
                    while cur != *next {
                        match parent.get(&cur) {
                            Some(p) => {
                                cur = p.clone();
                                if cur != *next {
                                    cycle.push(cur.clone());
                                }
                            }
                            None => break,
                        }
                    }
                    cycle.push(next.to_string());
                    cycle.reverse();
                    return Some(cycle);
                }
                _ => {}
            }
        }
    }
    color.insert(start.to_string(), 2);
    None
}

fn weakly_connected(
    nodes: &[String],
    undirected: &HashMap<String, Vec<String>>,
) -> Vec<Vec<String>> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for s in nodes {
        if visited.contains(s) {
            continue;
        }
        let mut component = Vec::new();
        let mut stack = vec![s.clone()];
        while let Some(node) = stack.pop() {
            if !visited.insert(node.clone()) {
                continue;
            }
            component.push(node.clone());
            if let Some(neighbors) = undirected.get(&node) {
                for n in neighbors {
                    if !visited.contains(n) {
                        stack.push(n.clone());
                    }
                }
            }
        }
        component.sort();
        out.push(component);
    }
    // Sort components by their first member for deterministic output.
    out.sort_by(|a, b| a.first().cmp(&b.first()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use syn::parse_str;

    fn parse(src: &str) -> Pipeline {
        parse_str(src).expect("parse")
    }

    #[test]
    fn linear_pipeline_has_no_diagnostics() {
        let p = parse(indoc! {r#"
            stages: { a @ x::a, b @ x::b, c @ x::c },
            flow:   [a -> b, b -> c]
        "#});
        assert!(validate_pipeline(&p).is_empty());
    }

    #[test]
    fn cycle_without_cyclic_flag_is_error() {
        let p = parse(indoc! {r#"
            stages: { a @ x::a, b @ x::b, c @ x::c },
            flow:   [a -> b, b -> c, c -> a]
        "#});
        let d = validate_pipeline(&p);
        assert_eq!(d.len(), 1);
        match &d[0].kind {
            PipelineDiagKind::UnpermittedCycle { cycle } => {
                // Cycle should mention all three stages (closed path).
                assert!(cycle.contains(&"a".to_string()));
                assert!(cycle.contains(&"b".to_string()));
                assert!(cycle.contains(&"c".to_string()));
            }
            other => panic!("wrong kind: {other:?}"),
        }
    }

    #[test]
    fn cycle_with_cyclic_flag_is_ok() {
        let p = parse(indoc! {r#"
            stages: { a @ x::a, b @ x::b },
            flow:   [a -> b, b -> a],
            cyclic: true
        "#});
        assert!(validate_pipeline(&p).is_empty());
    }

    #[test]
    fn self_loop_is_a_cycle() {
        let p = parse(indoc! {r#"
            stages: { a @ x::a, b @ x::b },
            flow:   [a -> b, a -> a]
        "#});
        let d = validate_pipeline(&p);
        assert!(d
            .iter()
            .any(|x| matches!(x.kind, PipelineDiagKind::UnpermittedCycle { .. })));
    }

    #[test]
    fn unconnected_stage_is_reported() {
        let p = parse(indoc! {r#"
            stages: { a @ x::a, b @ x::b, lonely @ x::lonely },
            flow:   [a -> b]
        "#});
        let d = validate_pipeline(&p);
        assert_eq!(d.len(), 1);
        match &d[0].kind {
            PipelineDiagKind::UnconnectedStage { stage } => assert_eq!(stage, "lonely"),
            other => panic!("wrong kind: {other:?}"),
        }
    }

    #[test]
    fn two_disconnected_subchains_reported() {
        let p = parse(indoc! {r#"
            stages: { a @ x::a, b @ x::b, c @ x::c, d @ x::d },
            flow:   [a -> b, c -> d]
        "#});
        let d = validate_pipeline(&p);
        // No unconnected stages (every stage is in some edge); 2 components.
        match &d[0].kind {
            PipelineDiagKind::DisconnectedComponents { components } => {
                assert_eq!(components.len(), 2);
            }
            other => panic!("wrong kind: {other:?}"),
        }
    }

    #[test]
    fn diamond_is_one_component_no_cycle() {
        let p = parse(indoc! {r#"
            stages: { a @ x::a, b @ x::b, c @ x::c, d @ x::d },
            flow:   [a -> b, a -> c, b -> d, c -> d]
        "#});
        assert!(validate_pipeline(&p).is_empty());
    }
}
