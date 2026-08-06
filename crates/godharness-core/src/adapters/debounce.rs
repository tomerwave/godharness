use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    #[serde(default)]
    pub prompt_count: u32,
    #[serde(default)]
    pub last_injected_at: BTreeMap<String, u32>,
}

pub fn should_inject(state: &SessionState, standard_id: &str, reinject_after_prompts: u32) -> bool {
    if reinject_after_prompts == 0 {
        return true;
    }
    match state.last_injected_at.get(standard_id) {
        None => true,
        Some(&last) => state.prompt_count.saturating_sub(last) >= reinject_after_prompts,
    }
}

pub fn record_injection(state: &mut SessionState, standard_id: &str) {
    state
        .last_injected_at
        .insert(standard_id.to_string(), state.prompt_count);
}
