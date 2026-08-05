use std::path::Path;

use crate::standard::{Standard, StandardError, parse_standard};

const RECOMMENDED_V1_DOCUMENTS: &[(&str, &str)] = &[
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
    (
        "suites/recommended/v1/prefer-existing-solutions.md",
        include_str!("../suites/recommended/v1/prefer-existing-solutions.md"),
    ),
    (
        "suites/recommended/v1/design-for-extension.md",
        include_str!("../suites/recommended/v1/design-for-extension.md"),
    ),
    (
        "suites/recommended/v1/simplify-before-done.md",
        include_str!("../suites/recommended/v1/simplify-before-done.md"),
    ),
    (
        "suites/recommended/v1/verify-through-real-path.md",
        include_str!("../suites/recommended/v1/verify-through-real-path.md"),
    ),
    (
        "suites/recommended/v1/automate-everything.md",
        include_str!("../suites/recommended/v1/automate-everything.md"),
    ),
];

pub fn recommended_v1() -> Result<Vec<Standard>, StandardError> {
    RECOMMENDED_V1_DOCUMENTS
        .iter()
        .map(|(path, document)| parse_standard(document, Path::new(path)))
        .collect()
}
