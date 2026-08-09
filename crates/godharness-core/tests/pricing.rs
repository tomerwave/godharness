use godharness_core::{
    UsageEvent, UsageKind, estimate_cost, rate_for, snapshot_fetched_at, snapshot_source,
};

fn standard_event(id: &str, approx_tokens: u32, model: Option<&str>) -> UsageEvent {
    UsageEvent {
        timestamp_unix: 0,
        kind: UsageKind::Standard,
        id: id.to_string(),
        approx_tokens,
        model: model.map(str::to_string),
    }
}

#[test]
fn rate_for_finds_a_known_anthropic_model() {
    let rate = rate_for("claude-sonnet-4-6").expect("claude-sonnet-4-6 should be priced");
    assert!(rate.input > 0.0);
    assert!(rate.output > rate.input);
}

#[test]
fn rate_for_returns_none_for_an_unknown_model() {
    assert_eq!(rate_for("not-a-real-model-id"), None);
}

#[test]
fn snapshot_metadata_is_non_empty() {
    assert!(!snapshot_source().is_empty());
    assert!(!snapshot_fetched_at().is_empty());
}

#[test]
fn estimate_cost_prices_events_by_their_own_detected_model() {
    let events = vec![standard_event("a", 1_000_000, Some("claude-sonnet-4-6"))];

    let report = estimate_cost(&events, None);

    assert_eq!(report.priced.len(), 1);
    assert_eq!(report.priced[0].model, "claude-sonnet-4-6");
    assert_eq!(report.priced[0].tokens, 1_000_000);
    assert_eq!(report.priced[0].estimated_usd, 3.0);
    assert_eq!(report.unpriced_tokens, 0);
}

#[test]
fn estimate_cost_excludes_events_with_no_detected_model() {
    let events = vec![
        standard_event("a", 1_000_000, Some("claude-sonnet-4-6")),
        standard_event("b", 500_000, None),
    ];

    let report = estimate_cost(&events, None);

    assert_eq!(report.priced.len(), 1);
    assert_eq!(report.unpriced_tokens, 500_000);
}

#[test]
fn estimate_cost_treats_an_unknown_model_as_unpriced() {
    let events = vec![standard_event("a", 1_000_000, Some("not-a-real-model-id"))];

    let report = estimate_cost(&events, None);

    assert!(report.priced.is_empty());
    assert_eq!(report.unpriced_tokens, 1_000_000);
}

#[test]
fn estimate_cost_with_override_prices_every_event_as_the_override_model() {
    let events = vec![
        standard_event("a", 1_000_000, Some("claude-haiku-4-5")),
        standard_event("b", 1_000_000, None),
    ];

    let report = estimate_cost(&events, Some("claude-sonnet-4-6"));

    assert_eq!(report.priced.len(), 1);
    assert_eq!(report.priced[0].model, "claude-sonnet-4-6");
    assert_eq!(report.priced[0].tokens, 2_000_000);
    assert_eq!(report.priced[0].estimated_usd, 6.0);
    assert_eq!(
        report.unpriced_tokens, 0,
        "override applies even to events with no detected model"
    );
}

#[test]
fn estimate_cost_groups_multiple_models_separately() {
    let events = vec![
        standard_event("a", 1_000_000, Some("claude-sonnet-4-6")),
        standard_event("b", 1_000_000, Some("claude-haiku-4-5")),
    ];

    let report = estimate_cost(&events, None);

    assert_eq!(report.priced.len(), 2);
    let sonnet = report
        .priced
        .iter()
        .find(|entry| entry.model == "claude-sonnet-4-6")
        .expect("sonnet entry");
    let haiku = report
        .priced
        .iter()
        .find(|entry| entry.model == "claude-haiku-4-5")
        .expect("haiku entry");
    assert_eq!(sonnet.estimated_usd, 3.0);
    assert_eq!(haiku.estimated_usd, 1.0);
}
