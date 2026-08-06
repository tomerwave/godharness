use serde::Serialize;

use crate::graph::StandardGraph;
use crate::resolve::{ResolvedStandard, resolve};

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

fn is_must_read(graph: &StandardGraph, resolved: &ResolvedStandard) -> bool {
    graph
        .standard(&resolved.id)
        .is_some_and(|standard| standard.must_read)
}

fn resolve_for_event(
    graph: &StandardGraph,
    event: ClaudeCodeEvent,
    prompt: Option<&str>,
) -> Vec<ResolvedStandard> {
    match event {
        ClaudeCodeEvent::UserPromptSubmit => resolve(graph, prompt, &[])
            .into_iter()
            .filter(|resolved| !is_must_read(graph, resolved))
            .collect(),
        ClaudeCodeEvent::SessionStart => resolve(graph, None, &[]),
    }
}

pub fn claude_code_hook_response(
    graph: &StandardGraph,
    event: ClaudeCodeEvent,
    prompt: Option<&str>,
) -> Option<String> {
    let resolved = resolve_for_event(graph, event, prompt);
    if resolved.is_empty() {
        return None;
    }
    let output = HookOutput {
        hook_specific_output: HookSpecificOutput {
            hook_event_name: event.name().to_string(),
            additional_context: format_context(&resolved),
        },
    };
    serde_json::to_string(&output).ok()
}
