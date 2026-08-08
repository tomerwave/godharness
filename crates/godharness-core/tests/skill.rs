use godharness_core::parse_skill;
use std::path::Path;

#[test]
fn parses_a_valid_skill_document() {
    let document = "---\nname: atomic-commits\ndescription: Split changes into single-purpose commits.\n---\n\n# Atomic Commits\n\nBody text here.\n";

    let skill = parse_skill(
        document,
        "atomic-commits",
        Path::new("skills/atomic-commits/SKILL.md"),
    )
    .expect("valid skill should parse");

    assert_eq!(skill.id, "atomic-commits");
    assert_eq!(skill.name, "atomic-commits");
    assert_eq!(
        skill.description,
        "Split changes into single-purpose commits."
    );
    assert_eq!(skill.body, "# Atomic Commits\n\nBody text here.");
}

#[test]
fn rejects_a_document_without_frontmatter() {
    let document = "# No Frontmatter\n\nJust prose.\n";

    let result = parse_skill(
        document,
        "no-frontmatter",
        Path::new("skills/no-frontmatter/SKILL.md"),
    );

    assert!(result.is_err());
}

#[test]
fn rejects_a_document_missing_description() {
    let document = "---\nname: incomplete\n---\n\nBody.\n";

    let result = parse_skill(
        document,
        "incomplete",
        Path::new("skills/incomplete/SKILL.md"),
    );

    assert!(result.is_err());
}
