use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use crate::skill::{Skill, parse_skill};
use crate::standard::{Standard, StandardError, parse_standard};

const STANDARDS: &[(&str, &str, &str)] = &[
    (
        "naming",
        "standards/naming.md",
        include_str!("../standards/naming.md"),
    ),
    (
        "small-focused-units",
        "standards/small-focused-units.md",
        include_str!("../standards/small-focused-units.md"),
    ),
    (
        "error-handling",
        "standards/error-handling.md",
        include_str!("../standards/error-handling.md"),
    ),
    (
        "secrets-and-security",
        "standards/secrets-and-security.md",
        include_str!("../standards/secrets-and-security.md"),
    ),
    (
        "configuration-boundaries",
        "standards/configuration-boundaries.md",
        include_str!("../standards/configuration-boundaries.md"),
    ),
    (
        "testing",
        "standards/testing.md",
        include_str!("../standards/testing.md"),
    ),
    (
        "runtime-validation",
        "standards/runtime-validation.md",
        include_str!("../standards/runtime-validation.md"),
    ),
    (
        "architecture-decisions",
        "standards/architecture-decisions.md",
        include_str!("../standards/architecture-decisions.md"),
    ),
    (
        "prefer-existing-solutions",
        "standards/prefer-existing-solutions.md",
        include_str!("../standards/prefer-existing-solutions.md"),
    ),
    (
        "design-for-extension",
        "standards/design-for-extension.md",
        include_str!("../standards/design-for-extension.md"),
    ),
    (
        "simplify-before-done",
        "standards/simplify-before-done.md",
        include_str!("../standards/simplify-before-done.md"),
    ),
    (
        "verify-through-real-path",
        "standards/verify-through-real-path.md",
        include_str!("../standards/verify-through-real-path.md"),
    ),
    (
        "automate-everything",
        "standards/automate-everything.md",
        include_str!("../standards/automate-everything.md"),
    ),
    (
        "dependency-direction",
        "standards/dependency-direction.md",
        include_str!("../standards/dependency-direction.md"),
    ),
    (
        "interface-segregation",
        "standards/interface-segregation.md",
        include_str!("../standards/interface-segregation.md"),
    ),
    (
        "liskov-substitutability",
        "standards/liskov-substitutability.md",
        include_str!("../standards/liskov-substitutability.md"),
    ),
    (
        "single-level-of-abstraction",
        "standards/single-level-of-abstraction.md",
        include_str!("../standards/single-level-of-abstraction.md"),
    ),
    (
        "dont-repeat-yourself",
        "standards/dont-repeat-yourself.md",
        include_str!("../standards/dont-repeat-yourself.md"),
    ),
    (
        "no-broken-windows",
        "standards/no-broken-windows.md",
        include_str!("../standards/no-broken-windows.md"),
    ),
    (
        "ubiquitous-language",
        "standards/ubiquitous-language.md",
        include_str!("../standards/ubiquitous-language.md"),
    ),
    (
        "test-pyramid-shape",
        "standards/test-pyramid-shape.md",
        include_str!("../standards/test-pyramid-shape.md"),
    ),
    (
        "test-independence",
        "standards/test-independence.md",
        include_str!("../standards/test-independence.md"),
    ),
    (
        "flaky-test-is-signal",
        "standards/flaky-test-is-signal.md",
        include_str!("../standards/flaky-test-is-signal.md"),
    ),
    (
        "injection-parameterize",
        "standards/injection-parameterize.md",
        include_str!("../standards/injection-parameterize.md"),
    ),
    (
        "server-side-authorization",
        "standards/server-side-authorization.md",
        include_str!("../standards/server-side-authorization.md"),
    ),
    (
        "pin-and-verify-dependencies",
        "standards/pin-and-verify-dependencies.md",
        include_str!("../standards/pin-and-verify-dependencies.md"),
    ),
    (
        "test-data-builders",
        "standards/test-data-builders.md",
        include_str!("../standards/test-data-builders.md"),
    ),
    (
        "structured-logging-over-printf",
        "standards/structured-logging-over-printf.md",
        include_str!("../standards/structured-logging-over-printf.md"),
    ),
    (
        "small-reviewable-changes",
        "standards/small-reviewable-changes.md",
        include_str!("../standards/small-reviewable-changes.md"),
    ),
    (
        "leave-code-cleaner-than-you-found-it",
        "standards/leave-code-cleaner-than-you-found-it.md",
        include_str!("../standards/leave-code-cleaner-than-you-found-it.md"),
    ),
    (
        "concise-communication",
        "standards/concise-communication.md",
        include_str!("../standards/concise-communication.md"),
    ),
];

static PARSED_STANDARDS: LazyLock<HashMap<&'static str, Result<Standard, StandardError>>> =
    LazyLock::new(|| {
        STANDARDS
            .iter()
            .map(|(id, path, document)| (*id, parse_standard(document, Path::new(path))))
            .collect()
    });

pub fn standard(id: &str) -> Result<Standard, StandardError> {
    PARSED_STANDARDS
        .get(id)
        .unwrap_or_else(|| {
            panic!(
                "registry: unknown standard id {id:?} — this is a bug in suite.rs, not user input"
            )
        })
        .clone()
}

const SKILLS: &[(&str, &str, &str)] = &[
    (
        "isolate-refactoring-from-behavior-change",
        "skills/isolate-refactoring-from-behavior-change/SKILL.md",
        include_str!("../skills/isolate-refactoring-from-behavior-change/SKILL.md"),
    ),
    (
        "property-based-testing",
        "skills/property-based-testing/SKILL.md",
        include_str!("../skills/property-based-testing/SKILL.md"),
    ),
    (
        "atomic-commits",
        "skills/atomic-commits/SKILL.md",
        include_str!("../skills/atomic-commits/SKILL.md"),
    ),
    (
        "resource-oriented-api-design",
        "skills/resource-oriented-api-design/SKILL.md",
        include_str!("../skills/resource-oriented-api-design/SKILL.md"),
    ),
    (
        "systematic-debugging",
        "skills/systematic-debugging/SKILL.md",
        include_str!("../skills/systematic-debugging/SKILL.md"),
    ),
    (
        "verification-before-completion",
        "skills/verification-before-completion/SKILL.md",
        include_str!("../skills/verification-before-completion/SKILL.md"),
    ),
    (
        "requesting-code-review",
        "skills/requesting-code-review/SKILL.md",
        include_str!("../skills/requesting-code-review/SKILL.md"),
    ),
    (
        "receiving-code-review",
        "skills/receiving-code-review/SKILL.md",
        include_str!("../skills/receiving-code-review/SKILL.md"),
    ),
    (
        "simplify",
        "skills/simplify/SKILL.md",
        include_str!("../skills/simplify/SKILL.md"),
    ),
    (
        "ai-slop-cleaner",
        "skills/ai-slop-cleaner/SKILL.md",
        include_str!("../skills/ai-slop-cleaner/SKILL.md"),
    ),
    (
        "research-with-evidence",
        "skills/research-with-evidence/SKILL.md",
        include_str!("../skills/research-with-evidence/SKILL.md"),
    ),
    (
        "frontend-design",
        "skills/frontend-design/SKILL.md",
        include_str!("../skills/frontend-design/SKILL.md"),
    ),
    (
        "retrospective-workflow-review",
        "skills/retrospective-workflow-review/SKILL.md",
        include_str!("../skills/retrospective-workflow-review/SKILL.md"),
    ),
];

static PARSED_SKILLS: LazyLock<HashMap<&'static str, Skill>> = LazyLock::new(|| {
    SKILLS
        .iter()
        .map(|(id, path, document)| {
            let skill = parse_skill(document, id, Path::new(path)).unwrap_or_else(|error| {
                panic!("registry: embedded skill {id:?} failed to parse: {error}")
            });
            (*id, skill)
        })
        .collect()
});

pub fn skill(id: &str) -> Skill {
    PARSED_SKILLS
        .get(id)
        .unwrap_or_else(|| {
            panic!("registry: unknown skill id {id:?} — this is a bug in suite.rs, not user input")
        })
        .clone()
}
