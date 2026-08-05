use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use petgraph::Direction;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{EdgeFiltered, EdgeRef};

use crate::standard::Standard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Supersedes,
    RelatesTo,
}

#[derive(Debug)]
pub struct GraphError(String);

impl std::fmt::Display for GraphError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid standard graph: {}", self.0)
    }
}

impl std::error::Error for GraphError {}

pub struct StandardGraph {
    graph: DiGraph<Standard, EdgeKind>,
    index_by_id: HashMap<String, NodeIndex>,
}

impl StandardGraph {
    pub fn standard(&self, id: &str) -> Option<&Standard> {
        self.index_by_id.get(id).map(|index| &self.graph[*index])
    }

    pub fn is_superseded(&self, id: &str) -> bool {
        self.index_by_id.get(id).is_some_and(|&index| {
            self.graph
                .edges_directed(index, Direction::Incoming)
                .any(|edge| *edge.weight() == EdgeKind::Supersedes)
        })
    }

    pub fn related_to(&self, id: &str) -> Vec<&Standard> {
        let Some(&index) = self.index_by_id.get(id) else {
            return Vec::new();
        };
        self.graph
            .edges_directed(index, Direction::Outgoing)
            .filter(|edge| *edge.weight() == EdgeKind::RelatesTo)
            .map(|edge| &self.graph[edge.target()])
            .collect()
    }

    pub fn len(&self) -> usize {
        self.graph.node_count()
    }

    pub fn is_empty(&self) -> bool {
        self.graph.node_count() == 0
    }
}

fn add_edges(
    graph: &mut DiGraph<Standard, EdgeKind>,
    index_by_id: &HashMap<String, NodeIndex>,
    source_id: &str,
    target_ids: &[String],
    kind: EdgeKind,
    relationship_name: &str,
) -> Result<(), GraphError> {
    let source_index = index_by_id[source_id];

    for target_id in target_ids {
        let target_index = index_by_id.get(target_id).ok_or_else(|| {
            GraphError(format!(
                "{source_id} {relationship_name} unknown standard {target_id}"
            ))
        })?;
        graph.add_edge(source_index, *target_index, kind);
    }

    Ok(())
}

pub fn build_graph(standards: Vec<Standard>) -> Result<StandardGraph, GraphError> {
    let mut graph = DiGraph::new();
    let mut index_by_id = HashMap::new();

    for standard in standards {
        let id = standard.id.clone();
        if index_by_id.contains_key(&id) {
            return Err(GraphError(format!("duplicate standard id: {id}")));
        }
        let index = graph.add_node(standard);
        index_by_id.insert(id, index);
    }

    let ids: Vec<String> = index_by_id.keys().cloned().collect();
    for id in &ids {
        let index = index_by_id[id];
        let supersedes = graph[index].supersedes.clone();
        let relates_to = graph[index].relates_to.clone();

        add_edges(
            &mut graph,
            &index_by_id,
            id,
            &supersedes,
            EdgeKind::Supersedes,
            "supersedes",
        )?;
        add_edges(
            &mut graph,
            &index_by_id,
            id,
            &relates_to,
            EdgeKind::RelatesTo,
            "relates to",
        )?;
    }

    let supersedes_only =
        EdgeFiltered::from_fn(&graph, |edge| *edge.weight() == EdgeKind::Supersedes);
    if petgraph::algo::is_cyclic_directed(&supersedes_only) {
        return Err(GraphError(
            "supersedes relationships contain a cycle".to_string(),
        ));
    }

    Ok(StandardGraph { graph, index_by_id })
}

pub fn content_hash(documents: &[String]) -> u64 {
    let mut hasher = DefaultHasher::new();
    documents.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn standard(id: &str, supersedes: &[&str], relates_to: &[&str]) -> Standard {
        Standard {
            id: id.to_string(),
            title: id.to_string(),
            keywords: Vec::new(),
            paths: Vec::new(),
            must_read: false,
            supersedes: supersedes.iter().map(|s| s.to_string()).collect(),
            relates_to: relates_to.iter().map(|s| s.to_string()).collect(),
            rule: "Do it.".to_string(),
            why: None,
            how_to_apply: None,
            source_path: Path::new("x.md").to_path_buf(),
        }
    }

    #[test]
    fn links_supersedes_and_relates_to_edges() {
        let standards = vec![
            standard("new-rule", &["old-rule"], &["sibling-rule"]),
            standard("old-rule", &[], &[]),
            standard("sibling-rule", &[], &[]),
        ];

        let graph = build_graph(standards).expect("graph should build");

        assert!(graph.is_superseded("old-rule"));
        assert!(!graph.is_superseded("new-rule"));
        assert_eq!(
            graph
                .related_to("new-rule")
                .iter()
                .map(|s| s.id.as_str())
                .collect::<Vec<_>>(),
            vec!["sibling-rule"]
        );
        assert_eq!(graph.len(), 3);
    }

    #[test]
    fn rejects_duplicate_ids() {
        let standards = vec![standard("dup", &[], &[]), standard("dup", &[], &[])];

        let result = build_graph(standards);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_dangling_supersedes_reference() {
        let standards = vec![standard("a", &["missing"], &[])];

        let result = build_graph(standards);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_dangling_relates_to_reference() {
        let standards = vec![standard("a", &[], &["missing"])];

        let result = build_graph(standards);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_a_supersedes_cycle() {
        let standards = vec![standard("a", &["b"], &[]), standard("b", &["a"], &[])];

        let result = build_graph(standards);

        assert!(result.is_err());
    }

    #[test]
    fn allows_a_relates_to_cycle() {
        let standards = vec![standard("a", &[], &["b"]), standard("b", &[], &["a"])];

        let result = build_graph(standards);

        assert!(result.is_ok());
    }

    #[test]
    fn content_hash_changes_when_documents_change() {
        let first = content_hash(&["one".to_string()]);
        let second = content_hash(&["two".to_string()]);
        let repeat = content_hash(&["one".to_string()]);

        assert_ne!(first, second);
        assert_eq!(first, repeat);
    }
}
