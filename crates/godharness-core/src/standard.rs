use std::path::{Path, PathBuf};

use glob::Pattern;
use gray_matter::Matter;
use gray_matter::engine::YAML;
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

#[derive(Debug, Clone)]
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

fn extract_section(body: &str, heading_name: &str) -> Option<String> {
    let heading = format!("## {heading_name}");
    let after_heading = body.find(&heading)? + heading.len();
    let remainder = &body[after_heading..];
    let section_end = remainder.find("\n## ").unwrap_or(remainder.len());
    let section = remainder[..section_end].trim();
    (!section.is_empty()).then(|| section.to_string())
}

pub fn parse_standard(document: &str, source_path: &Path) -> Result<Standard, StandardError> {
    let parsed = Matter::<YAML>::new()
        .parse::<Frontmatter>(document)
        .map_err(|error| StandardError(error.to_string()))?;
    let frontmatter = parsed
        .data
        .ok_or_else(|| StandardError("document must start with a frontmatter block".to_string()))?;
    let rule = extract_section(&parsed.content, "Rule")
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
        why: extract_section(&parsed.content, "Why"),
        how_to_apply: extract_section(&parsed.content, "How to apply"),
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
