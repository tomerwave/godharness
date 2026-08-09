use serde::Serialize;

use crate::adapters::debounce::{SessionState, record_injection, should_inject};
use crate::graph::StandardGraph;
use crate::resolve::{
    ResolvedSkill, ResolvedStandard, resolve, resolve_by_keyword_only, resolve_skills,
    resolve_skills_by_keyword_only,
};
use crate::skill::Skill;

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

fn format_context(standards: &[ResolvedStandard], skills: &[ResolvedSkill]) -> String {
    let mut sections = Vec::new();
    if !standards.is_empty() {
        let lines: Vec<String> = standards
            .iter()
            .map(|standard| format!("- {}: {}", standard.id, standard.rule))
            .collect();
        sections.push(format!(
            "Relevant project standard(s):\n{}",
            lines.join("\n")
        ));
    }
    if !skills.is_empty() {
        let lines: Vec<String> = skills
            .iter()
            .map(|skill| format!("- {}: {}", skill.name, skill.description))
            .collect();
        sections.push(format!(
            "Skill(s) that may help with this — consider invoking them:\n{}",
            lines.join("\n")
        ));
    }
    sections.join("\n\n")
}

fn debounced_standards(
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

fn debounced_skills(
    state: &mut SessionState,
    skills: Vec<ResolvedSkill>,
    reinject_after_prompts: u32,
) -> Vec<ResolvedSkill> {
    let mut kept = Vec::new();
    for skill in skills {
        let key = format!("skill:{}", skill.id);
        if should_inject(state, &key, reinject_after_prompts) {
            record_injection(state, &key);
            kept.push(skill);
        }
    }
    kept
}

fn resolve_for_event(
    graph: &StandardGraph,
    skills: &[Skill],
    request: &HookRequest,
    state: &mut SessionState,
) -> (Vec<ResolvedStandard>, Vec<ResolvedSkill>) {
    match request.event {
        ClaudeCodeEvent::UserPromptSubmit => {
            state.prompt_count += 1;
            let matched_standards = request
                .prompt
                .map(|prompt| resolve_by_keyword_only(graph, prompt))
                .unwrap_or_default();
            let matched_skills = request
                .prompt
                .map(|prompt| resolve_skills_by_keyword_only(skills, prompt))
                .unwrap_or_default();
            (
                debounced_standards(state, matched_standards, request.reinject_after_prompts),
                debounced_skills(state, matched_skills, request.reinject_after_prompts),
            )
        }
        ClaudeCodeEvent::SessionStart => {
            (resolve(graph, None, &[]), resolve_skills(skills, None, &[]))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HookResult {
    pub response: Option<String>,
    pub standards: Vec<ResolvedStandard>,
    pub skills: Vec<ResolvedSkill>,
}

pub fn claude_code_hook_response(
    graph: &StandardGraph,
    skills: &[Skill],
    request: HookRequest,
    state: &mut SessionState,
) -> HookResult {
    let (resolved_standards, resolved_skills) = resolve_for_event(graph, skills, &request, state);
    if resolved_standards.is_empty() && resolved_skills.is_empty() {
        return HookResult {
            response: None,
            standards: resolved_standards,
            skills: resolved_skills,
        };
    }
    let output = HookOutput {
        hook_specific_output: HookSpecificOutput {
            hook_event_name: request.event.name().to_string(),
            additional_context: format_context(&resolved_standards, &resolved_skills),
        },
    };
    HookResult {
        response: serde_json::to_string(&output).ok(),
        standards: resolved_standards,
        skills: resolved_skills,
    }
}
