use std::path::Path;

use crate::standard::{Standard, StandardError, parse_standard};

const RECOMMENDED_V1_DOCUMENTS: &[(&str, &str)] = &[
    (
        "suites/recommended/v1/no-comments.md",
        include_str!("../suites/recommended/v1/no-comments.md"),
    ),
    (
        "suites/recommended/v1/naming.md",
        include_str!("../suites/recommended/v1/naming.md"),
    ),
    (
        "suites/recommended/v1/small-focused-units.md",
        include_str!("../suites/recommended/v1/small-focused-units.md"),
    ),
    (
        "suites/recommended/v1/error-handling.md",
        include_str!("../suites/recommended/v1/error-handling.md"),
    ),
    (
        "suites/recommended/v1/secrets-and-security.md",
        include_str!("../suites/recommended/v1/secrets-and-security.md"),
    ),
    (
        "suites/recommended/v1/configuration-boundaries.md",
        include_str!("../suites/recommended/v1/configuration-boundaries.md"),
    ),
    (
        "suites/recommended/v1/testing.md",
        include_str!("../suites/recommended/v1/testing.md"),
    ),
    (
        "suites/recommended/v1/runtime-validation.md",
        include_str!("../suites/recommended/v1/runtime-validation.md"),
    ),
    (
        "suites/recommended/v1/architecture-decisions.md",
        include_str!("../suites/recommended/v1/architecture-decisions.md"),
    ),
];

pub fn recommended_v1() -> Result<Vec<Standard>, StandardError> {
    RECOMMENDED_V1_DOCUMENTS
        .iter()
        .map(|(path, document)| parse_standard(document, Path::new(path)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::build_graph;

    #[test]
    fn every_embedded_document_parses() {
        let standards = recommended_v1().expect("every embedded document should parse");

        assert_eq!(standards.len(), RECOMMENDED_V1_DOCUMENTS.len());
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

        assert_eq!(graph.len(), RECOMMENDED_V1_DOCUMENTS.len());
    }

    #[test]
    fn must_read_standards_are_no_comments_and_secrets() {
        let standards = recommended_v1().expect("every embedded document should parse");

        let must_read: Vec<&str> = standards
            .iter()
            .filter(|standard| standard.must_read)
            .map(|standard| standard.id.as_str())
            .collect();

        assert_eq!(must_read, vec!["no-comments", "secrets-and-security"]);
    }
}
