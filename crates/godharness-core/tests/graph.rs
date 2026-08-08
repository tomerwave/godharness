use godharness_core::{Standard, build_graph, content_hash};

mod common;

fn standard(id: &str, supersedes: &[&str], relates_to: &[&str]) -> Standard {
    Standard {
        supersedes: supersedes.iter().map(|s| s.to_string()).collect(),
        relates_to: relates_to.iter().map(|s| s.to_string()).collect(),
        ..common::standard(id, &[], &[], false)
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
