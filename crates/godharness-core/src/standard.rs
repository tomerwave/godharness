use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use glob::Pattern;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use regex::Regex;
use serde::Deserialize;

use crate::error::string_error;

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

string_error!(StandardError, "invalid standard document: ");

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

fn compiled_keyword_regex(keyword: &str) -> Option<Regex> {
    static CACHE: OnceLock<Mutex<HashMap<String, Regex>>> = OnceLock::new();
    let mut cache = CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(compiled) = cache.get(keyword) {
        return Some(compiled.clone());
    }
    let pattern = format!(r"(?i)\b{}\b", regex::escape(keyword));
    let compiled = Regex::new(&pattern).ok()?;
    cache.insert(keyword.to_string(), compiled.clone());
    Some(compiled)
}

fn word_boundary_match(text: &str, keyword: &str) -> bool {
    compiled_keyword_regex(keyword).is_some_and(|compiled| compiled.is_match(text))
}

pub fn keyword_matches(prompt: &str, keywords: &[String]) -> bool {
    keywords
        .iter()
        .any(|keyword| word_boundary_match(prompt, keyword))
}

fn compiled_glob(glob: &str) -> Option<Pattern> {
    static CACHE: OnceLock<Mutex<HashMap<String, Pattern>>> = OnceLock::new();
    let mut cache = CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(compiled) = cache.get(glob) {
        return Some(compiled.clone());
    }
    let compiled = Pattern::new(glob).ok()?;
    cache.insert(glob.to_string(), compiled.clone());
    Some(compiled)
}

pub fn path_matches(file_path: &str, globs: &[String]) -> bool {
    globs
        .iter()
        .any(|glob| compiled_glob(glob).is_some_and(|compiled| compiled.matches(file_path)))
}
