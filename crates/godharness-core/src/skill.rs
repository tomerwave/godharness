use std::path::{Path, PathBuf};

use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;

use crate::error::string_error;

#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub body: String,
    pub keywords: Vec<String>,
    pub paths: Vec<String>,
    pub source_path: PathBuf,
}

string_error!(SkillError, "invalid skill document: ");

#[derive(Debug, Deserialize)]
struct Frontmatter {
    name: String,
    description: String,
    #[serde(default)]
    keywords: Vec<String>,
    #[serde(default)]
    paths: Vec<String>,
}

pub fn parse_skill(document: &str, id: &str, source_path: &Path) -> Result<Skill, SkillError> {
    let parsed = Matter::<YAML>::new()
        .parse::<Frontmatter>(document)
        .map_err(|error| SkillError(error.to_string()))?;
    let frontmatter = parsed
        .data
        .ok_or_else(|| SkillError("document must start with a frontmatter block".to_string()))?;

    Ok(Skill {
        id: id.to_string(),
        name: frontmatter.name,
        description: frontmatter.description,
        body: parsed.content.trim().to_string(),
        keywords: frontmatter.keywords,
        paths: frontmatter.paths,
        source_path: source_path.to_path_buf(),
    })
}
