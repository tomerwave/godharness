use crate::registry;
use crate::skill::Skill;
use crate::standard::{Standard, StandardError};

pub struct SuiteManifest {
    pub standards: &'static [&'static str],
    pub skills: &'static [&'static str],
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
        "leave-code-cleaner-than-you-found-it",
        "concise-communication",
    ],
    skills: &[
        "isolate-refactoring-from-behavior-change",
        "property-based-testing",
        "atomic-commits",
        "resource-oriented-api-design",
        "systematic-debugging",
        "verification-before-completion",
        "requesting-code-review",
        "receiving-code-review",
        "simplify",
        "ai-slop-cleaner",
        "research-with-evidence",
        "frontend-design",
        "retrospective-workflow-review",
        "clarify-before-building",
        "doubt-driven-development",
        "define-goal",
        "refactor-ux-01-frame-session",
        "refactor-ux-02-confirm-goal",
        "refactor-ux-03-draft-journeys",
        "refactor-ux-04-icp-panel-journeys",
        "refactor-ux-05-color-scheme",
        "refactor-ux-06-mock-iterate",
        "refactor-ux-07-consolidate",
        "refactor-ux-08-framework-doctor",
        "refactor-ux-09-build",
        "refactor-ux-10-icp-panel-look",
        "refactor-ux-11-final-pass",
        "refactor-ux-12-finalize-docs",
        "design-new-01-design-doc",
        "design-new-02-frontend-design",
        "design-new-03-icp-panel-look",
    ],
};

pub fn recommended_v1() -> Result<Vec<Standard>, StandardError> {
    RECOMMENDED_V1
        .standards
        .iter()
        .map(|id| registry::standard(id))
        .collect()
}

pub fn recommended_v1_skills() -> Vec<Skill> {
    RECOMMENDED_V1
        .skills
        .iter()
        .map(|id| registry::skill(id))
        .collect()
}
