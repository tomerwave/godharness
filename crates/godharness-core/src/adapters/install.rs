use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

use crate::error::string_error;

string_error!(
    InstallError,
    "",
    from: std::io::Error, serde_json::Error, serde_yaml::Error, crate::init::InitError, crate::check::CheckError,
);

#[derive(Debug, PartialEq)]
pub struct InstallReport {
    pub godharness_yaml_updated: bool,
    pub hook_config_updated: bool,
    pub hook_config_path: PathBuf,
    pub skills_installed: Vec<String>,
}

struct ToolProfile {
    yaml_key: &'static str,
    hook_config_relative_path: &'static str,
    skills_relative_path: &'static str,
}

fn tool_profile(tool_arg: &str) -> Result<ToolProfile, InstallError> {
    match tool_arg {
        "claude-code" => Ok(ToolProfile {
            yaml_key: "claude-code",
            hook_config_relative_path: ".claude/settings.json",
            skills_relative_path: ".claude/skills",
        }),
        "codex" => Ok(ToolProfile {
            yaml_key: "codex",
            hook_config_relative_path: ".codex/hooks.json",
            skills_relative_path: ".agents/skills",
        }),
        other => Err(InstallError(format!("unknown adapter tool: {other}"))),
    }
}

fn hook_command(tool_arg: &str, event: &str) -> String {
    format!("godharness adapter-hook {tool_arg} --event {event}")
}

fn read_json_document(path: &Path) -> Result<Value, InstallError> {
    if !path.exists() {
        return Ok(json!({}));
    }
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}

fn as_object<'a>(
    value: &'a mut Value,
    context: &str,
) -> Result<&'a mut Map<String, Value>, InstallError> {
    value
        .as_object_mut()
        .ok_or_else(|| InstallError(format!("{context} must be a JSON object")))
}

fn entries_for_event<'a>(
    document: &'a mut Value,
    event: &str,
) -> Result<&'a mut Vec<Value>, InstallError> {
    let root = as_object(document, "hook config")?;
    let hooks = root.entry("hooks").or_insert_with(|| json!({}));
    let hooks_object = as_object(hooks, "hooks")?;
    let entries = hooks_object
        .entry(event.to_string())
        .or_insert_with(|| json!([]));
    entries
        .as_array_mut()
        .ok_or_else(|| InstallError(format!("hooks.{event} must be a JSON array")))
}

fn command_present(entries: &[Value], command: &str) -> bool {
    entries.iter().any(|entry| {
        entry
            .get("hooks")
            .and_then(Value::as_array)
            .is_some_and(|hooks| hooks.iter().any(|hook| hook["command"] == command))
    })
}

fn add_hook_entry(document: &mut Value, event: &str, command: &str) -> Result<bool, InstallError> {
    let entries = entries_for_event(document, event)?;
    if command_present(entries, command) {
        return Ok(false);
    }
    entries.push(json!({ "hooks": [ { "type": "command", "command": command } ] }));
    Ok(true)
}

fn add_hook_entries(document: &mut Value, tool_arg: &str) -> Result<bool, InstallError> {
    let submit_added = add_hook_entry(
        document,
        "UserPromptSubmit",
        &hook_command(tool_arg, "user-prompt-submit"),
    )?;
    let start_added = add_hook_entry(
        document,
        "SessionStart",
        &hook_command(tool_arg, "session-start"),
    )?;
    let stop_added = add_hook_entry(document, "Stop", &hook_command(tool_arg, "stop"))?;
    Ok(submit_added || start_added || stop_added)
}

fn write_hook_config(path: &Path, document: &Value) -> Result<(), InstallError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(
        path,
        format!("{}\n", serde_json::to_string_pretty(document)?),
    )?;
    Ok(())
}

fn merge_hook_config(path: &Path, tool_arg: &str) -> Result<bool, InstallError> {
    let mut document = read_json_document(path)?;
    let updated = add_hook_entries(&mut document, tool_arg)?;
    if updated {
        write_hook_config(path, &document)?;
    }
    Ok(updated)
}

fn as_yaml_mapping<'a>(
    value: &'a mut serde_yaml::Value,
    context: &str,
) -> Result<&'a mut serde_yaml::Mapping, InstallError> {
    value
        .as_mapping_mut()
        .ok_or_else(|| InstallError(format!("{context} must be a YAML mapping")))
}

fn add_adapter_key(document: &mut serde_yaml::Value, yaml_key: &str) -> Result<bool, InstallError> {
    let mapping = as_yaml_mapping(document, "godharness.yaml")?;
    let adapters_key = serde_yaml::Value::from("adapters");
    let mut adapters = match mapping.get(&adapters_key) {
        Some(serde_yaml::Value::Mapping(existing)) => existing.clone(),
        _ => serde_yaml::Mapping::new(),
    };
    let key = serde_yaml::Value::from(yaml_key);
    if adapters.get(&key) == Some(&serde_yaml::Value::Bool(true)) {
        return Ok(false);
    }
    adapters.insert(key, serde_yaml::Value::Bool(true));
    mapping.insert(adapters_key, serde_yaml::Value::Mapping(adapters));
    Ok(true)
}

fn read_yaml_document(path: &Path) -> Result<serde_yaml::Value, InstallError> {
    Ok(serde_yaml::from_str(&std::fs::read_to_string(path)?)?)
}

fn write_yaml_if_updated(
    path: &Path,
    document: &serde_yaml::Value,
    updated: bool,
) -> Result<(), InstallError> {
    if !updated {
        return Ok(());
    }
    std::fs::write(path, serde_yaml::to_string(document)?)?;
    Ok(())
}

fn merge_yaml_adapter(root: &Path, yaml_key: &str) -> Result<bool, InstallError> {
    let path = root.join("godharness.yaml");
    let mut document = read_yaml_document(&path)?;
    let updated = add_adapter_key(&mut document, yaml_key)?;
    write_yaml_if_updated(&path, &document, updated)?;
    Ok(updated)
}

fn skill_document(skill: &crate::skill::Skill) -> String {
    format!(
        "---\nname: {}\ndescription: {}\n---\n\n{}\n",
        skill.name, skill.description, skill.body
    )
}

fn write_skill_if_changed(path: &Path, skill: &crate::skill::Skill) -> Result<bool, InstallError> {
    let rendered = skill_document(skill);
    if std::fs::read_to_string(path).ok().as_deref() == Some(rendered.as_str()) {
        return Ok(false);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, rendered)?;
    Ok(true)
}

fn install_skills(root: &Path, skills_relative_path: &str) -> Result<Vec<String>, InstallError> {
    let config = crate::check::load_config(root)?;
    let mut installed = Vec::new();

    for skill in crate::check::load_suite_skills(&config) {
        let path = root
            .join(skills_relative_path)
            .join(&skill.id)
            .join("SKILL.md");
        if write_skill_if_changed(&path, &skill)? {
            installed.push(skill.id.clone());
        }
    }

    Ok(installed)
}

struct AdapterInstallation {
    hook_config_updated: bool,
    hook_config_path: PathBuf,
    skills_installed: Vec<String>,
}

fn install_hook_and_skills(
    root: &Path,
    tool_arg: &str,
    profile: &ToolProfile,
) -> Result<AdapterInstallation, InstallError> {
    let hook_config_path = root.join(profile.hook_config_relative_path);
    let hook_config_updated = merge_hook_config(&hook_config_path, tool_arg)?;
    let skills_installed = install_skills(root, profile.skills_relative_path)?;
    Ok(AdapterInstallation {
        hook_config_updated,
        hook_config_path,
        skills_installed,
    })
}

pub fn enable_adapter(root: &Path, tool_arg: &str) -> Result<InstallReport, InstallError> {
    let profile = tool_profile(tool_arg)?;
    crate::init::run_init(root)?;
    let godharness_yaml_updated = merge_yaml_adapter(root, profile.yaml_key)?;
    let installation = install_hook_and_skills(root, tool_arg, &profile)?;
    Ok(InstallReport {
        godharness_yaml_updated,
        hook_config_updated: installation.hook_config_updated,
        hook_config_path: installation.hook_config_path,
        skills_installed: installation.skills_installed,
    })
}
