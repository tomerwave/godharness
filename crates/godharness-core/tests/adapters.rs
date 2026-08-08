use std::path::Path;

use godharness_core::{FieldMapping, render_shape_a, write_rendered_files};

mod common;
use common::{TempRoot, standard};

const TEST_MAPPING: FieldMapping = FieldMapping {
    scope_key: "globs",
    always_scope: "**/*",
    directory: ".rules",
    extension: "md",
};

#[test]
fn path_scoped_standard_renders_with_globs_key() {
    let standards = vec![standard("rust-only", &[], &["**/*.rs"], false)];

    let rendered = render_shape_a(&standards, &TEST_MAPPING).expect("render should succeed");

    assert_eq!(rendered.len(), 1);
    assert_eq!(rendered[0].path, Path::new(".rules/rust-only.md"));
    assert!(rendered[0].content.contains("globs:\n- '**/*.rs'"));
    assert!(!rendered[0].content.contains("paths:"));
}

#[test]
fn must_read_standard_renders_with_an_always_matching_glob() {
    let standards = vec![standard("always", &[], &[], true)];

    let rendered = render_shape_a(&standards, &TEST_MAPPING).expect("render should succeed");

    assert_eq!(rendered.len(), 1);
    assert!(rendered[0].content.contains("globs:\n- '**/*'"));
}

#[test]
fn keyword_only_standard_renders_nothing() {
    let standards = vec![standard("keyword-only", &["trigger"], &[], false)];

    let rendered = render_shape_a(&standards, &TEST_MAPPING).expect("render should succeed");

    assert!(rendered.is_empty());
}

#[test]
fn body_includes_rule_why_and_how_to_apply_when_present() {
    let mut full = standard("full", &[], &["**/*.rs"], false);
    full.why = Some("Because reasons.".to_string());
    full.how_to_apply = Some("Do the thing.".to_string());

    let rendered = render_shape_a(&[full], &TEST_MAPPING).expect("render should succeed");

    assert_eq!(rendered.len(), 1);
    let content = &rendered[0].content;
    assert!(content.contains("## Rule\n\nRule for full."));
    assert!(content.contains("## Why\n\nBecause reasons."));
    assert!(content.contains("## How to apply\n\nDo the thing."));
}

#[test]
fn body_omits_why_and_how_to_apply_when_absent() {
    let standards = vec![standard("minimal", &[], &["**/*.rs"], false)];

    let rendered = render_shape_a(&standards, &TEST_MAPPING).expect("render should succeed");

    let content = &rendered[0].content;
    assert!(!content.contains("## Why"));
    assert!(!content.contains("## How to apply"));
}

#[test]
fn a_title_with_yaml_special_characters_still_produces_valid_frontmatter() {
    let mut tricky = standard("tricky", &[], &["**/*.rs"], false);
    tricky.title = "Release: v1".to_string();

    let rendered = render_shape_a(&[tricky], &TEST_MAPPING).expect("render should succeed");

    assert_eq!(rendered.len(), 1);
    let parsed = gray_matter::Matter::<gray_matter::engine::YAML>::new()
        .parse::<serde_yaml::Value>(&rendered[0].content)
        .expect("generated frontmatter should parse as valid YAML");
    let data = parsed.data.expect("frontmatter should have parsed data");
    assert_eq!(data["title"].as_str(), Some("Release: v1"));
}

#[test]
fn a_glob_with_quotes_still_produces_valid_frontmatter() {
    let standards = vec![standard("quoted", &[], &["**/\"weird\"/**"], false)];

    let rendered = render_shape_a(&standards, &TEST_MAPPING).expect("render should succeed");

    assert_eq!(rendered.len(), 1);
    let parsed = gray_matter::Matter::<gray_matter::engine::YAML>::new()
        .parse::<serde_yaml::Value>(&rendered[0].content)
        .expect("generated frontmatter should parse as valid YAML");
    let data = parsed.data.expect("frontmatter should have parsed data");
    assert_eq!(data["globs"][0].as_str(), Some("**/\"weird\"/**"));
}

#[test]
fn write_rendered_files_creates_the_directory_and_file() {
    let root = TempRoot::new("godharness-adapters-test", "write");
    let standards = vec![standard("rust-only", &[], &["**/*.rs"], false)];
    let rendered = render_shape_a(&standards, &TEST_MAPPING).expect("render should succeed");

    write_rendered_files(&root.path, &rendered).expect("write should succeed");

    let written = std::fs::read_to_string(root.path.join(".rules/rust-only.md"))
        .expect("rendered file should exist");
    assert!(written.contains("globs:\n- '**/*.rs'"));
}

#[test]
fn write_rendered_files_regenerates_existing_content() {
    let root = TempRoot::new("godharness-adapters-test", "regenerate");
    std::fs::create_dir_all(root.path.join(".rules")).expect("create rules dir");
    std::fs::write(root.path.join(".rules/rust-only.md"), "stale content")
        .expect("write stale content");

    let standards = vec![standard("rust-only", &[], &["**/*.rs"], false)];
    let rendered = render_shape_a(&standards, &TEST_MAPPING).expect("render should succeed");
    write_rendered_files(&root.path, &rendered).expect("write should succeed");

    let written = std::fs::read_to_string(root.path.join(".rules/rust-only.md"))
        .expect("rendered file should exist");
    assert!(!written.contains("stale content"));
    assert!(written.contains("globs:\n- '**/*.rs'"));
}
