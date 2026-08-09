use godharness_core::{
    ClaudeCodeEvent, HookRequest, SessionState, Skill, Standard, build_graph,
    claude_code_hook_response,
};

mod common;

fn standard(id: &str, keywords: &[&str], must_read: bool) -> Standard {
    common::standard(id, keywords, &[], must_read)
}

fn skill(id: &str, keywords: &[&str]) -> Skill {
    Skill {
        id: id.to_string(),
        name: id.to_string(),
        description: format!("Description for {id}."),
        body: "Body.".to_string(),
        keywords: keywords.iter().map(|s| s.to_string()).collect(),
        paths: Vec::new(),
        source_path: std::path::Path::new(&format!("{id}/SKILL.md")).to_path_buf(),
    }
}

fn request(
    event: ClaudeCodeEvent,
    prompt: Option<&str>,
    reinject_after_prompts: u32,
) -> HookRequest<'_> {
    HookRequest {
        event,
        prompt,
        reinject_after_prompts,
    }
}

#[allow(clippy::expect_used)]
fn additional_context(response: &str) -> String {
    let parsed: serde_json::Value = serde_json::from_str(response).expect("valid JSON");
    parsed["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .expect("additionalContext should be a string")
        .to_string()
}

#[test]
fn user_prompt_submit_returns_json_when_a_keyword_matches() {
    let graph = build_graph(vec![standard("errors", &["error"], false)]).expect("graph builds");
    let mut state = SessionState::default();

    let response = claude_code_hook_response(
        &graph,
        &[],
        request(ClaudeCodeEvent::UserPromptSubmit, Some("found an error"), 0),
        &mut state,
    )
    .response
    .expect("a match should produce output");

    let parsed: serde_json::Value = serde_json::from_str(&response).expect("valid JSON");
    assert_eq!(
        parsed["hookSpecificOutput"]["hookEventName"],
        "UserPromptSubmit"
    );
    assert!(additional_context(&response).contains("errors: Rule for errors."));
}

#[test]
fn user_prompt_submit_returns_none_when_nothing_matches() {
    let graph = build_graph(vec![standard("errors", &["error"], false)]).expect("graph builds");
    let mut state = SessionState::default();

    let response = claude_code_hook_response(
        &graph,
        &[],
        request(ClaudeCodeEvent::UserPromptSubmit, Some("hello"), 0),
        &mut state,
    );

    assert_eq!(response.response, None);
}

#[test]
fn user_prompt_submit_matches_must_read_standards_by_keyword_too() {
    let graph =
        build_graph(vec![standard("secrets", &["credential"], true)]).expect("graph builds");
    let mut state = SessionState::default();

    let response = claude_code_hook_response(
        &graph,
        &[],
        request(
            ClaudeCodeEvent::UserPromptSubmit,
            Some("add a credential"),
            0,
        ),
        &mut state,
    )
    .response
    .expect("a keyword match on a must-read standard should still produce output");

    assert!(additional_context(&response).contains("secrets"));
}

#[test]
fn user_prompt_submit_ignores_must_read_when_no_keyword_matches() {
    let graph =
        build_graph(vec![standard("secrets", &["credential"], true)]).expect("graph builds");
    let mut state = SessionState::default();

    let response = claude_code_hook_response(
        &graph,
        &[],
        request(
            ClaudeCodeEvent::UserPromptSubmit,
            Some("unrelated wording"),
            0,
        ),
        &mut state,
    );

    assert_eq!(response.response, None);
}

#[test]
fn user_prompt_submit_repeats_on_every_matching_prompt_by_default() {
    let graph = build_graph(vec![standard("errors", &["error"], false)]).expect("graph builds");
    let mut state = SessionState::default();

    for _ in 0..3 {
        let response = claude_code_hook_response(
            &graph,
            &[],
            request(ClaudeCodeEvent::UserPromptSubmit, Some("found an error"), 0),
            &mut state,
        );
        assert!(response.response.is_some());
    }
}

#[test]
fn user_prompt_submit_debounces_repeats_within_the_configured_window() {
    let graph = build_graph(vec![standard("errors", &["error"], false)]).expect("graph builds");
    let mut state = SessionState::default();
    let prompt = Some("found an error");

    let first = claude_code_hook_response(
        &graph,
        &[],
        request(ClaudeCodeEvent::UserPromptSubmit, prompt, 3),
        &mut state,
    );
    assert!(first.response.is_some());

    let second = claude_code_hook_response(
        &graph,
        &[],
        request(ClaudeCodeEvent::UserPromptSubmit, prompt, 3),
        &mut state,
    );
    assert_eq!(second.response, None);

    let third = claude_code_hook_response(
        &graph,
        &[],
        request(ClaudeCodeEvent::UserPromptSubmit, prompt, 3),
        &mut state,
    );
    assert_eq!(third.response, None);

    let fourth = claude_code_hook_response(
        &graph,
        &[],
        request(ClaudeCodeEvent::UserPromptSubmit, prompt, 3),
        &mut state,
    );
    assert!(fourth.response.is_some());
}

#[test]
fn session_start_returns_only_must_read_standards() {
    let graph = build_graph(vec![
        standard("always", &[], true),
        standard("keyword-only", &["trigger"], false),
    ])
    .expect("graph builds");
    let mut state = SessionState::default();

    let response = claude_code_hook_response(
        &graph,
        &[],
        request(ClaudeCodeEvent::SessionStart, None, 0),
        &mut state,
    )
    .response
    .expect("must-read standards should produce output");

    let parsed: serde_json::Value = serde_json::from_str(&response).expect("valid JSON");
    assert_eq!(
        parsed["hookSpecificOutput"]["hookEventName"],
        "SessionStart"
    );
    let context = additional_context(&response);
    assert!(context.contains("always"));
    assert!(!context.contains("keyword-only"));
}

#[test]
fn session_start_returns_none_when_no_standard_is_must_read() {
    let graph =
        build_graph(vec![standard("keyword-only", &["trigger"], false)]).expect("graph builds");
    let mut state = SessionState::default();

    let response = claude_code_hook_response(
        &graph,
        &[],
        request(ClaudeCodeEvent::SessionStart, None, 0),
        &mut state,
    );

    assert_eq!(response.response, None);
}

#[test]
fn session_start_is_unaffected_by_debounce_state() {
    let graph = build_graph(vec![standard("always", &[], true)]).expect("graph builds");
    let mut state = SessionState {
        prompt_count: 1,
        ..Default::default()
    };
    state.last_injected_at.insert("always".to_string(), 0);

    let response = claude_code_hook_response(
        &graph,
        &[],
        request(ClaudeCodeEvent::SessionStart, None, 3),
        &mut state,
    );

    assert!(response.response.is_some());
}

#[test]
fn user_prompt_submit_nudges_a_skill_when_its_keyword_matches() {
    let graph = build_graph(vec![]).expect("graph builds");
    let skills = vec![skill("debugging", &["broken", "failing test"])];
    let mut state = SessionState::default();

    let response = claude_code_hook_response(
        &graph,
        &skills,
        request(ClaudeCodeEvent::UserPromptSubmit, Some("this is broken"), 0),
        &mut state,
    )
    .response
    .expect("a skill keyword match should produce output");

    let context = additional_context(&response);
    assert!(context.contains("Skill(s) that may help"));
    assert!(context.contains("debugging: Description for debugging."));
}

#[test]
fn user_prompt_submit_returns_none_when_no_skill_keyword_matches() {
    let graph = build_graph(vec![]).expect("graph builds");
    let skills = vec![skill("debugging", &["broken"])];
    let mut state = SessionState::default();

    let response = claude_code_hook_response(
        &graph,
        &skills,
        request(
            ClaudeCodeEvent::UserPromptSubmit,
            Some("unrelated wording"),
            0,
        ),
        &mut state,
    );

    assert_eq!(response.response, None);
}

#[test]
fn skill_nudges_debounce_independently_of_a_same_named_standard() {
    let graph =
        build_graph(vec![standard("shared-id", &["trigger"], false)]).expect("graph builds");
    let skills = vec![skill("shared-id", &["trigger"])];
    let mut state = SessionState::default();
    let prompt = Some("trigger this");

    let first = claude_code_hook_response(
        &graph,
        &skills,
        request(ClaudeCodeEvent::UserPromptSubmit, prompt, 3),
        &mut state,
    )
    .response
    .expect("first prompt should match both the standard and the skill");
    assert!(additional_context(&first).contains("shared-id"));

    let second = claude_code_hook_response(
        &graph,
        &skills,
        request(ClaudeCodeEvent::UserPromptSubmit, prompt, 3),
        &mut state,
    );
    assert_eq!(
        second.response, None,
        "both the standard and the skill should be debounced on the second prompt"
    );
}

#[test]
fn session_start_never_nudges_skills() {
    let graph = build_graph(vec![]).expect("graph builds");
    let skills = vec![skill("debugging", &[])];
    let mut state = SessionState::default();

    let response = claude_code_hook_response(
        &graph,
        &skills,
        request(ClaudeCodeEvent::SessionStart, None, 0),
        &mut state,
    );

    assert_eq!(response.response, None);
}
