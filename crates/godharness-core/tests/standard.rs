use std::path::Path;

use godharness_core::{keyword_matches, parse_standard, path_matches};

fn sample_document() -> String {
    "\
---
id: no-comments
title: No Code Comments
keywords: [comment, comments]
paths: [\"**/*.rs\"]
must-read: true
supersedes: [old-comment-rule]
relates-to: [small-focused-units]
---

## Rule

Never write comments.

## Why

Names should carry the meaning instead.

## How to apply

Rename before you comment.
"
    .to_string()
}

#[test]
fn parses_a_full_standard_document() {
    let standard = parse_standard(
        &sample_document(),
        Path::new("docs/godharness/no-comments.md"),
    )
    .expect("valid standard should parse");

    assert_eq!(standard.id, "no-comments");
    assert_eq!(standard.title, "No Code Comments");
    assert_eq!(
        standard.keywords,
        vec!["comment".to_string(), "comments".to_string()]
    );
    assert_eq!(standard.paths, vec!["**/*.rs".to_string()]);
    assert!(standard.must_read);
    assert_eq!(standard.supersedes, vec!["old-comment-rule".to_string()]);
    assert_eq!(standard.relates_to, vec!["small-focused-units".to_string()]);
    assert_eq!(standard.rule, "Never write comments.");
    assert_eq!(
        standard.why.as_deref(),
        Some("Names should carry the meaning instead.")
    );
    assert_eq!(
        standard.how_to_apply.as_deref(),
        Some("Rename before you comment.")
    );
}

#[test]
fn rejects_a_document_without_frontmatter() {
    let result = parse_standard("## Rule\n\nDo the thing.\n", Path::new("x.md"));

    assert!(result.is_err());
}

#[test]
fn rejects_a_document_without_a_rule_section() {
    let document = "---\nid: x\ntitle: X\n---\n\n## Why\n\nBecause.\n";

    let result = parse_standard(document, Path::new("x.md"));

    assert!(result.is_err());
}

#[test]
fn defaults_are_empty_and_false_when_omitted() {
    let document = "---\nid: x\ntitle: X\n---\n\n## Rule\n\nDo it.\n";

    let standard =
        parse_standard(document, Path::new("x.md")).expect("valid standard should parse");

    assert!(standard.keywords.is_empty());
    assert!(standard.paths.is_empty());
    assert!(!standard.must_read);
    assert!(standard.supersedes.is_empty());
    assert!(standard.relates_to.is_empty());
    assert!(standard.why.is_none());
    assert!(standard.how_to_apply.is_none());
}

#[test]
fn keyword_match_respects_word_boundaries() {
    let keywords = vec!["error".to_string()];

    assert!(keyword_matches("found an error here", &keywords));
    assert!(!keyword_matches("found errors here", &keywords));
    assert!(!keyword_matches("a terror story", &keywords));
}

#[test]
fn keyword_match_is_case_insensitive() {
    let keywords = vec!["error".to_string()];

    assert!(keyword_matches("An ERROR occurred", &keywords));
}

#[test]
fn path_match_supports_recursive_globs() {
    let globs = vec!["**/*.rs".to_string()];

    assert!(path_matches("crates/godharness-core/src/lib.rs", &globs));
    assert!(!path_matches("crates/godharness-core/src/lib.ts", &globs));
}
