use std::path::{Path, PathBuf};

use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub body: String,
    pub source_path: PathBuf,
}

#[derive(Debug)]
pub struct SkillError(String);

impl std::fmt::Display for SkillError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid skill document: {}", self.0)
    }
}

impl std::error::Error for SkillError {}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    name: String,
    description: String,
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
        source_path: source_path.to_path_buf(),
    })
}
