use std::path::Path;

use godharness_core::{ClaudeCodeEvent, Standard, build_graph, claude_code_hook_response};

fn standard(id: &str, keywords: &[&str], must_read: bool) -> Standard {
    Standard {
        id: id.to_string(),
        title: id.to_string(),
        keywords: keywords.iter().map(|s| s.to_string()).collect(),
        paths: Vec::new(),
        must_read,
        supersedes: Vec::new(),
        relates_to: Vec::new(),
        rule: format!("Rule for {id}."),
        why: None,
        how_to_apply: None,
        source_path: Path::new(&format!("{id}.md")).to_path_buf(),
    }
}

#[test]
fn user_prompt_submit_returns_json_when_a_keyword_matches() {
    let graph = build_graph(vec![standard("errors", &["error"], false)]).expect("graph builds");

    let response = claude_code_hook_response(
        &graph,
        ClaudeCodeEvent::UserPromptSubmit,
        Some("found an error"),
    )
    .expect("a match should produce output");

    let parsed: serde_json::Value = serde_json::from_str(&response).expect("valid JSON");
    assert_eq!(
        parsed["hookSpecificOutput"]["hookEventName"],
        "UserPromptSubmit"
    );
    assert!(
        parsed["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .expect("additionalContext should be a string")
            .contains("errors: Rule for errors.")
    );
}

#[test]
fn user_prompt_submit_returns_none_when_nothing_matches() {
    let graph = build_graph(vec![standard("errors", &["error"], false)]).expect("graph builds");

    let response =
        claude_code_hook_response(&graph, ClaudeCodeEvent::UserPromptSubmit, Some("hello"));

    assert_eq!(response, None);
}

#[test]
fn user_prompt_submit_excludes_must_read_standards_even_on_every_prompt() {
    let graph = build_graph(vec![
        standard("always", &[], true),
        standard("errors", &["error"], false),
    ])
    .expect("graph builds");

    let response = claude_code_hook_response(
        &graph,
        ClaudeCodeEvent::UserPromptSubmit,
        Some("found an error"),
    )
    .expect("the keyword match should still produce output");

    let parsed: serde_json::Value = serde_json::from_str(&response).expect("valid JSON");
    let context = parsed["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .expect("additionalContext should be a string");
    assert!(context.contains("errors"));
    assert!(!context.contains("always"));
}

#[test]
fn user_prompt_submit_returns_none_when_only_a_must_read_standard_exists() {
    let graph = build_graph(vec![standard("always", &[], true)]).expect("graph builds");

    let response = claude_code_hook_response(
        &graph,
        ClaudeCodeEvent::UserPromptSubmit,
        Some("anything at all"),
    );

    assert_eq!(response, None);
}

#[test]
fn session_start_returns_only_must_read_standards() {
    let graph = build_graph(vec![
        standard("always", &[], true),
        standard("keyword-only", &["trigger"], false),
    ])
    .expect("graph builds");

    let response = claude_code_hook_response(&graph, ClaudeCodeEvent::SessionStart, None)
        .expect("must-read standards should produce output");

    let parsed: serde_json::Value = serde_json::from_str(&response).expect("valid JSON");
    assert_eq!(
        parsed["hookSpecificOutput"]["hookEventName"],
        "SessionStart"
    );
    let context = parsed["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .expect("additionalContext should be a string");
    assert!(context.contains("always"));
    assert!(!context.contains("keyword-only"));
}

#[test]
fn session_start_returns_none_when_no_standard_is_must_read() {
    let graph =
        build_graph(vec![standard("keyword-only", &["trigger"], false)]).expect("graph builds");

    let response = claude_code_hook_response(&graph, ClaudeCodeEvent::SessionStart, None);

    assert_eq!(response, None);
}
