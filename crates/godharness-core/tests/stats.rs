use godharness_core::{
    StatEntry, UsageEvent, UsageKind, aggregate, append_events, approx_tokens, read_events,
    usage_log_path,
};

mod common;
use common::TempRoot;

fn temp_root(name: &str) -> TempRoot {
    TempRoot::new("godharness-core-test-stats", name)
}

#[test]
fn approx_tokens_estimates_roughly_four_characters_per_token() {
    assert_eq!(approx_tokens(""), 0);
    assert_eq!(approx_tokens("abcd"), 1);
    assert_eq!(approx_tokens("abcdefgh"), 2);
    assert_eq!(approx_tokens("abcde"), 2);
}

#[test]
fn usage_log_path_is_deterministic_and_repo_specific() {
    let home = std::path::Path::new("/home/example");
    let repo_a = std::path::Path::new("/repos/a");
    let repo_b = std::path::Path::new("/repos/b");

    let path_a = usage_log_path(home, repo_a);
    let path_a_again = usage_log_path(home, repo_a);
    let path_b = usage_log_path(home, repo_b);

    assert_eq!(path_a, path_a_again);
    assert_ne!(path_a, path_b);
    assert!(path_a.starts_with(home.join(".godharness").join("usage")));
    assert_eq!(
        path_a.extension().and_then(|ext| ext.to_str()),
        Some("jsonl")
    );
}

#[test]
fn append_and_read_events_round_trips() {
    let root = temp_root("round-trip");
    let path = root.path.join("usage.jsonl");

    let events = vec![
        UsageEvent {
            timestamp_unix: 100,
            kind: UsageKind::Standard,
            id: "secrets-and-security".to_string(),
            approx_tokens: 12,
        },
        UsageEvent {
            timestamp_unix: 101,
            kind: UsageKind::Skill,
            id: "systematic-debugging".to_string(),
            approx_tokens: 30,
        },
    ];

    append_events(&path, &events).expect("append should succeed");
    let read_back = read_events(&path);

    assert_eq!(read_back, events);
}

#[test]
fn append_events_is_cumulative_across_calls() {
    let root = temp_root("cumulative");
    let path = root.path.join("usage.jsonl");

    append_events(
        &path,
        &[UsageEvent {
            timestamp_unix: 1,
            kind: UsageKind::Standard,
            id: "a".to_string(),
            approx_tokens: 5,
        }],
    )
    .expect("first append should succeed");

    append_events(
        &path,
        &[UsageEvent {
            timestamp_unix: 2,
            kind: UsageKind::Standard,
            id: "a".to_string(),
            approx_tokens: 5,
        }],
    )
    .expect("second append should succeed");

    assert_eq!(read_events(&path).len(), 2);
}

#[test]
fn read_events_on_a_missing_file_is_an_empty_list() {
    let root = temp_root("missing");
    let path = root.path.join("does-not-exist.jsonl");

    assert_eq!(read_events(&path), Vec::new());
}

#[test]
fn aggregate_sums_tokens_and_counts_fires_per_id_sorted_by_cost() {
    let events = vec![
        UsageEvent {
            timestamp_unix: 1,
            kind: UsageKind::Standard,
            id: "small".to_string(),
            approx_tokens: 5,
        },
        UsageEvent {
            timestamp_unix: 2,
            kind: UsageKind::Standard,
            id: "big".to_string(),
            approx_tokens: 50,
        },
        UsageEvent {
            timestamp_unix: 3,
            kind: UsageKind::Standard,
            id: "small".to_string(),
            approx_tokens: 5,
        },
    ];

    let entries = aggregate(&events);

    assert_eq!(
        entries,
        vec![
            StatEntry {
                kind: UsageKind::Standard,
                id: "big".to_string(),
                fires: 1,
                total_approx_tokens: 50,
            },
            StatEntry {
                kind: UsageKind::Standard,
                id: "small".to_string(),
                fires: 2,
                total_approx_tokens: 10,
            },
        ]
    );
}

#[test]
fn aggregate_keeps_standards_and_skills_with_the_same_id_separate() {
    let events = vec![
        UsageEvent {
            timestamp_unix: 1,
            kind: UsageKind::Standard,
            id: "shared-id".to_string(),
            approx_tokens: 10,
        },
        UsageEvent {
            timestamp_unix: 2,
            kind: UsageKind::Skill,
            id: "shared-id".to_string(),
            approx_tokens: 20,
        },
    ];

    let entries = aggregate(&events);

    assert_eq!(entries.len(), 2);
    assert!(
        entries
            .iter()
            .any(|entry| entry.kind == UsageKind::Standard && entry.total_approx_tokens == 10)
    );
    assert!(
        entries
            .iter()
            .any(|entry| entry.kind == UsageKind::Skill && entry.total_approx_tokens == 20)
    );
}
