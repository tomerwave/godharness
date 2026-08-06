use serde::Serialize;

use crate::adapters::debounce::{SessionState, record_injection, should_inject};
use crate::graph::StandardGraph;
use crate::resolve::{ResolvedStandard, resolve, resolve_by_keyword_only};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaudeCodeEvent {
    UserPromptSubmit,
    SessionStart,
}

impl ClaudeCodeEvent {
    fn name(self) -> &'static str {
        match self {
            ClaudeCodeEvent::UserPromptSubmit => "UserPromptSubmit",
            ClaudeCodeEvent::SessionStart => "SessionStart",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HookRequest<'a> {
    pub event: ClaudeCodeEvent,
    pub prompt: Option<&'a str>,
    pub reinject_after_prompts: u32,
}

#[derive(Serialize)]
struct HookSpecificOutput {
    #[serde(rename = "hookEventName")]
    hook_event_name: String,
    #[serde(rename = "additionalContext")]
    additional_context: String,
}

#[derive(Serialize)]
struct HookOutput {
    #[serde(rename = "hookSpecificOutput")]
    hook_specific_output: HookSpecificOutput,
}

fn format_context(standards: &[ResolvedStandard]) -> String {
    let lines: Vec<String> = standards
        .iter()
        .map(|standard| format!("- {}: {}", standard.id, standard.rule))
        .collect();
    format!("Relevant project standard(s):\n{}", lines.join("\n"))
}

fn debounced(
    state: &mut SessionState,
    standards: Vec<ResolvedStandard>,
    reinject_after_prompts: u32,
) -> Vec<ResolvedStandard> {
    let mut kept = Vec::new();
    for standard in standards {
        if should_inject(state, &standard.id, reinject_after_prompts) {
            record_injection(state, &standard.id);
            kept.push(standard);
        }
    }
    kept
}

fn resolve_for_event(
    graph: &StandardGraph,
    request: &HookRequest,
    state: &mut SessionState,
) -> Vec<ResolvedStandard> {
    match request.event {
        ClaudeCodeEvent::UserPromptSubmit => {
            state.prompt_count += 1;
            let matched = request
                .prompt
                .map(|prompt| resolve_by_keyword_only(graph, prompt))
                .unwrap_or_default();
            debounced(state, matched, request.reinject_after_prompts)
        }
        ClaudeCodeEvent::SessionStart => resolve(graph, None, &[]),
    }
}

pub fn claude_code_hook_response(
    graph: &StandardGraph,
    request: HookRequest,
    state: &mut SessionState,
) -> Option<String> {
    let resolved = resolve_for_event(graph, &request, state);
    if resolved.is_empty() {
        return None;
    }
    let output = HookOutput {
        hook_specific_output: HookSpecificOutput {
            hook_event_name: request.event.name().to_string(),
            additional_context: format_context(&resolved),
        },
    };
    serde_json::to_string(&output).ok()
}
