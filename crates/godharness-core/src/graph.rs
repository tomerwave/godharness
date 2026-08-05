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

impl EdgeKind {
    fn relationship_name(self) -> &'static str {
        match self {
            EdgeKind::Supersedes => "supersedes",
            EdgeKind::RelatesTo => "relates to",
        }
    }
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

struct GraphBuilder {
    graph: DiGraph<Standard, EdgeKind>,
    index_by_id: HashMap<String, NodeIndex>,
}

impl GraphBuilder {
    fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            index_by_id: HashMap::new(),
        }
    }

    fn insert_node(&mut self, standard: Standard) -> Result<(), GraphError> {
        let id = standard.id.clone();
        if self.index_by_id.contains_key(&id) {
            return Err(GraphError(format!("duplicate standard id: {id}")));
        }
        let index = self.graph.add_node(standard);
        self.index_by_id.insert(id, index);
        Ok(())
    }

    fn insert_edges(
        &mut self,
        source_id: &str,
        target_ids: &[String],
        kind: EdgeKind,
    ) -> Result<(), GraphError> {
        let source_index = self.index_by_id[source_id];

        for target_id in target_ids {
            let target_index = self.index_by_id.get(target_id).ok_or_else(|| {
                GraphError(format!(
                    "{source_id} {} unknown standard {target_id}",
                    kind.relationship_name()
                ))
            })?;
            self.graph.add_edge(source_index, *target_index, kind);
        }

        Ok(())
    }

    fn insert_standard_edges(&mut self, id: &str) -> Result<(), GraphError> {
        let index = self.index_by_id[id];
        let supersedes = self.graph[index].supersedes.clone();
        let relates_to = self.graph[index].relates_to.clone();

        self.insert_edges(id, &supersedes, EdgeKind::Supersedes)?;
        self.insert_edges(id, &relates_to, EdgeKind::RelatesTo)
    }

    fn has_supersedes_cycle(&self) -> bool {
        let supersedes_only =
            EdgeFiltered::from_fn(&self.graph, |edge| *edge.weight() == EdgeKind::Supersedes);
        petgraph::algo::is_cyclic_directed(&supersedes_only)
    }

    fn finish(self) -> Result<StandardGraph, GraphError> {
        if self.has_supersedes_cycle() {
            return Err(GraphError(
                "supersedes relationships contain a cycle".to_string(),
            ));
        }
        Ok(StandardGraph {
            graph: self.graph,
            index_by_id: self.index_by_id,
        })
    }
}

pub fn build_graph(standards: Vec<Standard>) -> Result<StandardGraph, GraphError> {
    let mut builder = GraphBuilder::new();

    for standard in standards {
        builder.insert_node(standard)?;
    }

    let ids: Vec<String> = builder.index_by_id.keys().cloned().collect();
    for id in &ids {
        builder.insert_standard_edges(id)?;
    }

    builder.finish()
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
