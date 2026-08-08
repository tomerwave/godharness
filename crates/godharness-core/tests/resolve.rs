use godharness_core::{build_graph, resolve, resolve_by_keyword_only};

mod common;
use common::standard;

#[test]
fn resolved_entry_carries_the_full_json_contract() {
    let mut source = standard("source", &["trigger"], &[], false);
    source.relates_to = vec!["sibling".to_string()];
    let sibling = standard("sibling", &[], &[], false);

    let graph = build_graph(vec![source, sibling]).expect("graph should build");

    let resolved = resolve(&graph, Some("trigger word"), &[]);

    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].id, "source");
    assert_eq!(resolved[0].path, "source.md");
    assert_eq!(resolved[0].rule, "Rule for source.");
    assert_eq!(resolved[0].relates_to, vec!["sibling".to_string()]);
}

#[test]
fn matches_by_keyword() {
    let graph =
        build_graph(vec![standard("errors", &["error"], &[], false)]).expect("graph should build");

    let resolved = resolve(&graph, Some("found an error"), &[]);

    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].id, "errors");
}

#[test]
fn matches_by_path() {
    let graph = build_graph(vec![standard("rust-only", &[], &["**/*.rs"], false)])
        .expect("graph should build");

    let resolved = resolve(&graph, None, &["src/lib.rs".to_string()]);

    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].id, "rust-only");
}

#[test]
fn must_read_always_matches() {
    let graph = build_graph(vec![standard("always", &[], &[], true)]).expect("graph should build");

    let resolved = resolve(&graph, None, &[]);

    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].id, "always");
}

#[test]
fn no_match_returns_empty() {
    let graph = build_graph(vec![standard("unrelated", &["nothing"], &[], false)])
        .expect("graph should build");

    let resolved = resolve(&graph, Some("hello"), &[]);

    assert!(resolved.is_empty());
}

#[test]
fn superseded_standards_are_excluded() {
    let mut new_standard = standard("new", &["comment"], &[], false);
    new_standard.supersedes = vec!["old".to_string()];
    let old_standard = standard("old", &["comment"], &[], false);

    let graph = build_graph(vec![new_standard, old_standard]).expect("graph should build");

    let resolved = resolve(&graph, Some("a comment"), &[]);

    assert_eq!(
        resolved.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
        vec!["new"]
    );
}

#[test]
fn keyword_only_matches_a_must_read_standard_when_its_keyword_matches() {
    let graph = build_graph(vec![standard("secrets", &["credential"], &[], true)])
        .expect("graph should build");

    let resolved = resolve_by_keyword_only(&graph, "add a credential");

    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].id, "secrets");
}

#[test]
fn keyword_only_ignores_a_must_read_standard_with_no_keyword_match() {
    let graph = build_graph(vec![standard("secrets", &["credential"], &[], true)])
        .expect("graph should build");

    let resolved = resolve_by_keyword_only(&graph, "unrelated wording");

    assert!(resolved.is_empty());
}

#[test]
fn keyword_only_ignores_path_matches() {
    let graph = build_graph(vec![standard("rust-only", &[], &["**/*.rs"], false)])
        .expect("graph should build");

    let resolved = resolve_by_keyword_only(&graph, "src/lib.rs");

    assert!(resolved.is_empty());
}

#[test]
fn relates_to_is_surfaced_on_the_match() {
    let mut source = standard("source", &["trigger"], &[], false);
    source.relates_to = vec!["sibling".to_string()];
    let sibling = standard("sibling", &[], &[], false);

    let graph = build_graph(vec![source, sibling]).expect("graph should build");

    let resolved = resolve(&graph, Some("trigger word"), &[]);

    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].relates_to, vec!["sibling".to_string()]);
}
