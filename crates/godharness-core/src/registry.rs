use std::path::Path;

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
];

pub fn standard(id: &str) -> Result<Standard, StandardError> {
    let (_, path, document) = STANDARDS
        .iter()
        .find(|(entry_id, _, _)| *entry_id == id)
        .unwrap_or_else(|| {
            panic!(
                "registry: unknown standard id {id:?} — this is a bug in suite.rs, not user input"
            )
        });
    parse_standard(document, Path::new(path))
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
];

pub fn skill(id: &str) -> Skill {
    let (_, path, document) = SKILLS
        .iter()
        .find(|(entry_id, _, _)| *entry_id == id)
        .unwrap_or_else(|| {
            panic!("registry: unknown skill id {id:?} — this is a bug in suite.rs, not user input")
        });
    parse_skill(document, id, Path::new(path))
        .unwrap_or_else(|error| panic!("registry: embedded skill {id:?} failed to parse: {error}"))
}
