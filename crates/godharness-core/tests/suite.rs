use godharness_core::{build_graph, recommended_v1, recommended_v1_skills};

const RECOMMENDED_V1_STANDARD_COUNT: usize = 32;

#[test]
fn recommended_v1_ids_are_unchanged_by_the_registry_refactor() {
    let standards = recommended_v1().expect("every embedded document should parse");
    let mut ids: Vec<&str> = standards
        .iter()
        .map(|standard| standard.id.as_str())
        .collect();
    ids.sort_unstable();

    assert_eq!(
        ids,
        vec![
            "architecture-decisions",
            "automate-everything",
            "concise-communication",
            "configuration-boundaries",
            "context-compression-evaluation",
            "dependency-direction",
            "design-for-extension",
            "dont-repeat-yourself",
            "error-handling",
            "flaky-test-is-signal",
            "injection-parameterize",
            "interface-segregation",
            "leave-code-cleaner-than-you-found-it",
            "liskov-substitutability",
            "naming",
            "no-broken-windows",
            "pin-and-verify-dependencies",
            "prefer-existing-solutions",
            "runtime-validation",
            "secrets-and-security",
            "server-side-authorization",
            "simplify-before-done",
            "single-level-of-abstraction",
            "small-focused-units",
            "small-reviewable-changes",
            "structured-logging-over-printf",
            "test-data-builders",
            "test-independence",
            "test-pyramid-shape",
            "testing",
            "ubiquitous-language",
            "verify-through-real-path",
        ]
    );
}

#[test]
fn every_embedded_document_parses() {
    let standards = recommended_v1().expect("every embedded document should parse");

    assert_eq!(standards.len(), RECOMMENDED_V1_STANDARD_COUNT);
}

#[test]
fn embedded_documents_have_unique_ids() {
    let standards = recommended_v1().expect("every embedded document should parse");
    let mut ids: Vec<&str> = standards
        .iter()
        .map(|standard| standard.id.as_str())
        .collect();
    ids.sort_unstable();
    ids.dedup();

    assert_eq!(ids.len(), standards.len());
}

#[test]
fn embedded_documents_form_a_valid_graph() {
    let standards = recommended_v1().expect("every embedded document should parse");

    let graph = build_graph(standards).expect("recommended@1 must be a valid graph");

    assert_eq!(graph.len(), RECOMMENDED_V1_STANDARD_COUNT);
}

#[test]
fn must_read_standards_are_secrets_simplify_and_verify() {
    let standards = recommended_v1().expect("every embedded document should parse");

    let must_read: Vec<&str> = standards
        .iter()
        .filter(|standard| standard.must_read)
        .map(|standard| standard.id.as_str())
        .collect();

    assert_eq!(
        must_read,
        vec![
            "secrets-and-security",
            "simplify-before-done",
            "verify-through-real-path",
        ]
    );
}

#[test]
fn recommended_v1_skills_all_parse_with_unique_ids() {
    let skills = recommended_v1_skills();

    assert_eq!(skills.len(), 13);
    let mut ids: Vec<&str> = skills.iter().map(|skill| skill.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 13);
}

#[test]
fn recommended_v1_skills_have_non_empty_name_and_description() {
    let skills = recommended_v1_skills();

    for skill in &skills {
        assert!(
            !skill.name.is_empty(),
            "skill {} has an empty name",
            skill.id
        );
        assert!(
            !skill.description.is_empty(),
            "skill {} has an empty description",
            skill.id
        );
    }
}
