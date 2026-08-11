use godharness_core::enable_adapter;

mod common;
use common::TempRoot;

fn temp_root(name: &str) -> TempRoot {
    TempRoot::new("godharness-core-test-install", name)
}

#[allow(clippy::expect_used)]
fn read_json(path: &std::path::Path) -> serde_json::Value {
    let contents = std::fs::read_to_string(path).expect("read json file");
    serde_json::from_str(&contents).expect("parse json file")
}

#[allow(clippy::expect_used)]
fn read_yaml(path: &std::path::Path) -> serde_yaml::Value {
    let contents = std::fs::read_to_string(path).expect("read yaml file");
    serde_yaml::from_str(&contents).expect("parse yaml file")
}

#[test]
fn enabling_claude_code_writes_both_hook_events() {
    let root = temp_root("claude-code-fresh");

    let report = enable_adapter(&root.path, "claude-code").expect("enable adapter");

    assert!(report.godharness_yaml_updated);
    assert!(report.hook_config_updated);

    let settings = read_json(&root.path.join(".claude/settings.json"));
    assert_eq!(
        settings["hooks"]["UserPromptSubmit"][0]["hooks"][0]["command"],
        "godharness adapter-hook claude-code --event user-prompt-submit"
    );
    assert_eq!(
        settings["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "godharness adapter-hook claude-code --event session-start"
    );

    let config = read_yaml(&root.path.join("godharness.yaml"));
    assert_eq!(
        config["adapters"]["claude-code"],
        serde_yaml::Value::Bool(true)
    );
}

#[test]
fn enabling_codex_writes_to_codex_hooks_json() {
    let root = temp_root("codex-fresh");

    let report = enable_adapter(&root.path, "codex").expect("enable adapter");

    assert!(report.hook_config_updated);
    assert_eq!(report.hook_config_path, root.path.join(".codex/hooks.json"));

    let hooks = read_json(&root.path.join(".codex/hooks.json"));
    assert_eq!(
        hooks["hooks"]["UserPromptSubmit"][0]["hooks"][0]["command"],
        "godharness adapter-hook codex --event user-prompt-submit"
    );

    let config = read_yaml(&root.path.join("godharness.yaml"));
    assert_eq!(config["adapters"]["codex"], serde_yaml::Value::Bool(true));
}

#[test]
fn enabling_twice_does_not_duplicate_entries() {
    let root = temp_root("idempotent");

    enable_adapter(&root.path, "claude-code").expect("first enable");
    let second = enable_adapter(&root.path, "claude-code").expect("second enable");

    assert!(!second.godharness_yaml_updated);
    assert!(!second.hook_config_updated);

    let settings = read_json(&root.path.join(".claude/settings.json"));
    assert_eq!(
        settings["hooks"]["UserPromptSubmit"]
            .as_array()
            .expect("UserPromptSubmit should be an array")
            .len(),
        1
    );
}

#[test]
fn enabling_preserves_unrelated_hooks_and_top_level_keys() {
    let root = temp_root("preserve");
    std::fs::create_dir_all(root.path.join(".claude")).expect("create .claude dir");
    std::fs::write(
        root.path.join(".claude/settings.json"),
        r#"{"$schema":"https://example.com/schema.json","hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"some-other-tool"}]}]}}"#,
    )
    .expect("seed settings.json");

    enable_adapter(&root.path, "claude-code").expect("enable adapter");

    let settings = read_json(&root.path.join(".claude/settings.json"));
    assert_eq!(settings["$schema"], "https://example.com/schema.json");
    assert_eq!(
        settings["hooks"]["PreToolUse"][0]["hooks"][0]["command"],
        "some-other-tool"
    );
    assert_eq!(
        settings["hooks"]["UserPromptSubmit"][0]["hooks"][0]["command"],
        "godharness adapter-hook claude-code --event user-prompt-submit"
    );
}

#[test]
fn enabling_preserves_existing_godharness_yaml_content() {
    let root = temp_root("preserve-yaml");
    std::fs::write(
        root.path.join("godharness.yaml"),
        "version: 1\nsuites: [recommended@1]\nstandards:\n  - docs/engineering/**\n",
    )
    .expect("seed godharness.yaml");

    enable_adapter(&root.path, "claude-code").expect("enable adapter");

    let config = read_yaml(&root.path.join("godharness.yaml"));
    assert_eq!(config["standards"][0], "docs/engineering/**");
    assert_eq!(
        config["adapters"]["claude-code"],
        serde_yaml::Value::Bool(true)
    );
}

#[test]
fn enabling_an_unknown_tool_returns_an_error() {
    let root = temp_root("unknown-tool");

    let result = enable_adapter(&root.path, "not-a-real-tool");

    assert!(result.is_err());
}

#[test]
fn enabling_claude_code_installs_all_four_recommended_skills() {
    let root = temp_root("skills-claude-code");

    let report = enable_adapter(&root.path, "claude-code").expect("enable adapter");

    assert_eq!(report.skills_installed.len(), 16);
    assert!(
        root.path
            .join(".claude/skills/atomic-commits/SKILL.md")
            .exists()
    );
    let content = std::fs::read_to_string(root.path.join(".claude/skills/atomic-commits/SKILL.md"))
        .expect("read installed skill");
    assert!(content.contains("name: atomic-commits"));
}

#[test]
fn enabling_codex_installs_skills_under_dot_agents_skills() {
    let root = temp_root("skills-codex");

    let report = enable_adapter(&root.path, "codex").expect("enable adapter");

    assert_eq!(report.skills_installed.len(), 16);
    assert!(
        root.path
            .join(".agents/skills/property-based-testing/SKILL.md")
            .exists()
    );
}

#[test]
fn enabling_twice_does_not_rewrite_unchanged_skills() {
    let root = temp_root("skills-idempotent");

    enable_adapter(&root.path, "claude-code").expect("first enable");
    let second = enable_adapter(&root.path, "claude-code").expect("second enable");

    assert!(second.skills_installed.is_empty());
}

#[test]
fn enabling_does_not_touch_a_skill_directory_it_did_not_create() {
    let root = temp_root("skills-preserve-foreign");
    std::fs::create_dir_all(root.path.join(".claude/skills/my-own-skill")).expect("create dir");
    std::fs::write(
        root.path.join(".claude/skills/my-own-skill/SKILL.md"),
        "---\nname: my-own-skill\ndescription: not godharness's.\n---\n\nMine.\n",
    )
    .expect("write foreign skill");

    enable_adapter(&root.path, "claude-code").expect("enable adapter");

    let content = std::fs::read_to_string(root.path.join(".claude/skills/my-own-skill/SKILL.md"))
        .expect("foreign skill should be untouched");
    assert!(content.contains("Mine."));
}
