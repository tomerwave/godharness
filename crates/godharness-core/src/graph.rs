use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use petgraph::Direction;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{EdgeFiltered, EdgeRef};

use crate::error::string_error;
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

string_error!(GraphError, "invalid standard graph: ");

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

    pub fn standards(&self) -> impl Iterator<Item = &Standard> {
        self.graph.node_weights()
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
