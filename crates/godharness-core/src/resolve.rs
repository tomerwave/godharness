use serde::Serialize;

use crate::graph::StandardGraph;
use crate::standard::{Standard, keyword_matches, path_matches};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ResolvedStandard {
    pub id: String,
    pub path: String,
    pub rule: String,
    #[serde(rename = "relates-to")]
    pub relates_to: Vec<String>,
}

fn is_relevant(standard: &Standard, prompt: Option<&str>, paths: &[String]) -> bool {
    if standard.must_read {
        return true;
    }
    if prompt.is_some_and(|prompt| keyword_matches(prompt, &standard.keywords)) {
        return true;
    }
    paths.iter().any(|path| path_matches(path, &standard.paths))
}

fn to_resolved(graph: &StandardGraph, standard: &Standard) -> ResolvedStandard {
    ResolvedStandard {
        id: standard.id.clone(),
        path: standard.source_path.to_string_lossy().into_owned(),
        rule: standard.rule.clone(),
        relates_to: graph
            .related_to(&standard.id)
            .iter()
            .map(|related| related.id.clone())
            .collect(),
    }
}

pub fn resolve(
    graph: &StandardGraph,
    prompt: Option<&str>,
    paths: &[String],
) -> Vec<ResolvedStandard> {
    graph
        .standards()
        .filter(|standard| !graph.is_superseded(&standard.id))
        .filter(|standard| is_relevant(standard, prompt, paths))
        .map(|standard| to_resolved(graph, standard))
        .collect()
}

pub fn resolve_by_keyword_only(graph: &StandardGraph, prompt: &str) -> Vec<ResolvedStandard> {
    graph
        .standards()
        .filter(|standard| !graph.is_superseded(&standard.id))
        .filter(|standard| keyword_matches(prompt, &standard.keywords))
        .map(|standard| to_resolved(graph, standard))
        .collect()
}
