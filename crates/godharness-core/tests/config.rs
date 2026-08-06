use godharness_core::parse_config;

#[test]
fn parses_minimal_config() {
    let yaml = "version: 1\nsuites: [recommended@1]\n";

    let config = parse_config(yaml).expect("valid config should parse");

    assert_eq!(config.version, 1);
    assert_eq!(config.suites, vec!["recommended@1".to_string()]);
    assert!(config.standards.is_empty());
    assert!(config.adapters.is_empty());
    assert_eq!(config.reinject_after_prompts, 0);
}

#[test]
fn parses_reinject_after_prompts() {
    let yaml = "version: 1\nreinject-after-prompts: 5\n";

    let config = parse_config(yaml).expect("valid config should parse");

    assert_eq!(config.reinject_after_prompts, 5);
}

#[test]
fn parses_full_config() {
    let yaml = "\
version: 1
suites: [recommended@1]
standards:
  - docs/engineering/**
  - docs/architecture/**
adapters:
  codex: true
  claude-code: true
  pi: false
";

    let config = parse_config(yaml).expect("valid config should parse");

    assert_eq!(
        config.standards,
        vec![
            "docs/engineering/**".to_string(),
            "docs/architecture/**".to_string(),
        ]
    );
    assert_eq!(config.adapters.get("codex"), Some(&true));
    assert_eq!(config.adapters.get("pi"), Some(&false));
}

#[test]
fn rejects_config_missing_version() {
    let yaml = "suites: [recommended@1]\n";

    let result = parse_config(yaml);

    assert!(result.is_err());
}
