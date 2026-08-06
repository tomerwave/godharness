use std::path::{Path, PathBuf};

use crate::standard::Standard;

pub struct FieldMapping {
    pub scope_key: &'static str,
    pub always_scope: &'static str,
    pub directory: &'static str,
    pub extension: &'static str,
}

#[derive(Debug, PartialEq)]
pub struct RenderedFile {
    pub path: PathBuf,
    pub content: String,
}

#[derive(Debug)]
pub struct AdapterError(String);

impl std::fmt::Display for AdapterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::error::Error for AdapterError {}

impl From<std::io::Error> for AdapterError {
    fn from(error: std::io::Error) -> Self {
        AdapterError(error.to_string())
    }
}

impl From<serde_yaml::Error> for AdapterError {
    fn from(error: serde_yaml::Error) -> Self {
        AdapterError(error.to_string())
    }
}

fn scope_for(standard: &Standard, mapping: &FieldMapping) -> Option<Vec<String>> {
    if standard.must_read {
        Some(vec![mapping.always_scope.to_string()])
    } else if !standard.paths.is_empty() {
        Some(standard.paths.clone())
    } else {
        None
    }
}

fn render_frontmatter(
    standard: &Standard,
    scope: &[String],
    mapping: &FieldMapping,
) -> Result<String, AdapterError> {
    let mut frontmatter = serde_yaml::Mapping::new();
    frontmatter.insert("title".into(), standard.title.clone().into());
    frontmatter.insert(mapping.scope_key.into(), scope.to_vec().into());
    let yaml = serde_yaml::to_string(&frontmatter)?;
    Ok(format!("---\n{yaml}---\n"))
}

fn render_body(standard: &Standard) -> String {
    let mut body = format!("## Rule\n\n{}\n", standard.rule);
    if let Some(why) = &standard.why {
        body.push_str(&format!("\n## Why\n\n{why}\n"));
    }
    if let Some(how_to_apply) = &standard.how_to_apply {
        body.push_str(&format!("\n## How to apply\n\n{how_to_apply}\n"));
    }
    body
}

fn render_one(
    standard: &Standard,
    mapping: &FieldMapping,
) -> Result<Option<RenderedFile>, AdapterError> {
    let Some(scope) = scope_for(standard, mapping) else {
        return Ok(None);
    };
    let frontmatter = render_frontmatter(standard, &scope, mapping)?;
    let body = render_body(standard);
    Ok(Some(RenderedFile {
        path: Path::new(mapping.directory).join(format!("{}.{}", standard.id, mapping.extension)),
        content: format!("{frontmatter}\n{body}"),
    }))
}

pub fn render_shape_a(
    standards: &[Standard],
    mapping: &FieldMapping,
) -> Result<Vec<RenderedFile>, AdapterError> {
    standards
        .iter()
        .filter_map(|standard| render_one(standard, mapping).transpose())
        .collect()
}

pub fn write_rendered_files(root: &Path, files: &[RenderedFile]) -> Result<(), AdapterError> {
    for file in files {
        let destination = root.join(&file.path);
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&destination, &file.content)?;
    }
    Ok(())
}
