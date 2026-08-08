use crate::registry;
use crate::standard::{Standard, StandardError};

pub struct SuiteManifest {
    pub standards: &'static [&'static str],
}

pub const RECOMMENDED_V1: SuiteManifest = SuiteManifest {
    standards: &[
        "naming",
        "small-focused-units",
        "error-handling",
        "secrets-and-security",
        "configuration-boundaries",
        "testing",
        "runtime-validation",
        "architecture-decisions",
        "prefer-existing-solutions",
        "design-for-extension",
        "simplify-before-done",
        "verify-through-real-path",
        "automate-everything",
        "dependency-direction",
        "interface-segregation",
        "liskov-substitutability",
        "single-level-of-abstraction",
        "dont-repeat-yourself",
        "no-broken-windows",
        "ubiquitous-language",
        "test-pyramid-shape",
        "test-independence",
        "flaky-test-is-signal",
        "injection-parameterize",
        "server-side-authorization",
        "pin-and-verify-dependencies",
        "test-data-builders",
        "structured-logging-over-printf",
        "small-reviewable-changes",
    ],
};

pub fn recommended_v1() -> Result<Vec<Standard>, StandardError> {
    RECOMMENDED_V1
        .standards
        .iter()
        .map(|id| registry::standard(id))
        .collect()
}
