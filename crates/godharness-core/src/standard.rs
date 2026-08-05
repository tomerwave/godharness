use std::path::{Path, PathBuf};

use glob::Pattern;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Standard {
    pub id: String,
    pub title: String,
    pub keywords: Vec<String>,
    pub paths: Vec<String>,
    pub must_read: bool,
    pub supersedes: Vec<String>,
    pub relates_to: Vec<String>,
    pub rule: String,
    pub why: Option<String>,
    pub how_to_apply: Option<String>,
    pub source_path: PathBuf,
}

#[derive(Debug)]
pub struct StandardError(String);

impl std::fmt::Display for StandardError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid standard document: {}", self.0)
    }
}

impl std::error::Error for StandardError {}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    id: String,
    title: String,
    #[serde(default)]
    keywords: Vec<String>,
    #[serde(default)]
    paths: Vec<String>,
    #[serde(default, rename = "must-read")]
    must_read: bool,
    #[serde(default)]
    supersedes: Vec<String>,
    #[serde(default, rename = "relates-to")]
    relates_to: Vec<String>,
}

fn split_frontmatter(document: &str) -> Result<(&str, &str), StandardError> {
    let after_open = document
        .strip_prefix("---\n")
        .ok_or_else(|| StandardError("document must start with a frontmatter block".to_string()))?;
    let close_offset = after_open
        .find("\n---\n")
        .ok_or_else(|| StandardError("frontmatter block is not closed".to_string()))?;
    Ok((&after_open[..close_offset], &after_open[close_offset + 5..]))
}

fn extract_section(body: &str, heading_name: &str) -> Option<String> {
    let heading = format!("## {heading_name}");
    let after_heading = body.find(&heading)? + heading.len();
    let remainder = &body[after_heading..];
    let section_end = remainder.find("\n## ").unwrap_or(remainder.len());
    let section = remainder[..section_end].trim();
    (!section.is_empty()).then(|| section.to_string())
}

pub fn parse_standard(document: &str, source_path: &Path) -> Result<Standard, StandardError> {
    let (frontmatter_text, body) = split_frontmatter(document)?;
    let frontmatter: Frontmatter =
        serde_yaml::from_str(frontmatter_text).map_err(|error| StandardError(error.to_string()))?;
    let rule = extract_section(body, "Rule")
        .ok_or_else(|| StandardError("standard requires a Rule section".to_string()))?;

    Ok(Standard {
        id: frontmatter.id,
        title: frontmatter.title,
        keywords: frontmatter.keywords,
        paths: frontmatter.paths,
        must_read: frontmatter.must_read,
        supersedes: frontmatter.supersedes,
        relates_to: frontmatter.relates_to,
        rule,
        why: extract_section(body, "Why"),
        how_to_apply: extract_section(body, "How to apply"),
        source_path: source_path.to_path_buf(),
    })
}

fn word_boundary_match(text: &str, keyword: &str) -> bool {
    let pattern = format!(r"(?i)\b{}\b", regex::escape(keyword));
    Regex::new(&pattern)
        .map(|compiled| compiled.is_match(text))
        .unwrap_or(false)
}

pub fn keyword_matches(prompt: &str, keywords: &[String]) -> bool {
    keywords
        .iter()
        .any(|keyword| word_boundary_match(prompt, keyword))
}

pub fn path_matches(file_path: &str, globs: &[String]) -> bool {
    globs.iter().any(|glob| {
        Pattern::new(glob)
            .map(|compiled| compiled.matches(file_path))
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
