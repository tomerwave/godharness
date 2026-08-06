use godharness_core::adapters::debounce::{SessionState, record_injection, should_inject};

#[test]
fn a_standard_never_injected_before_should_always_inject() {
    let state = SessionState::default();

    assert!(should_inject(&state, "errors", 3));
}

#[test]
fn zero_reinject_after_prompts_disables_debouncing() {
    let mut state = SessionState {
        prompt_count: 5,
        ..Default::default()
    };
    record_injection(&mut state, "errors");

    assert!(should_inject(&state, "errors", 0));
}

#[test]
fn a_standard_is_suppressed_until_the_window_elapses() {
    let mut state = SessionState {
        prompt_count: 1,
        ..Default::default()
    };
    record_injection(&mut state, "errors");

    state.prompt_count = 2;
    assert!(!should_inject(&state, "errors", 3));

    state.prompt_count = 3;
    assert!(!should_inject(&state, "errors", 3));

    state.prompt_count = 4;
    assert!(should_inject(&state, "errors", 3));
}

#[test]
fn record_injection_tracks_the_current_prompt_count() {
    let mut state = SessionState {
        prompt_count: 7,
        ..Default::default()
    };

    record_injection(&mut state, "errors");

    assert_eq!(state.last_injected_at.get("errors"), Some(&7));
}

#[test]
fn different_standards_are_tracked_independently() {
    let mut state = SessionState {
        prompt_count: 1,
        ..Default::default()
    };
    record_injection(&mut state, "errors");

    state.prompt_count = 2;
    assert!(!should_inject(&state, "errors", 3));
    assert!(should_inject(&state, "secrets", 3));
}
