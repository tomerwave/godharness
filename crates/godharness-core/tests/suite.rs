use godharness_core::{build_graph, recommended_v1};

const RECOMMENDED_V1_STANDARD_COUNT: usize = 13;

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
