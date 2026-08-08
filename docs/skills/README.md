# Skills

The 4 skills godharness installs as part of `recommended@1` are real, spec-compliant
`SKILL.md` files under `crates/godharness-core/skills/` — that directory is the source of
truth, not this one. `godharness adapters enable`/`update` write them into whichever tool a
repo has enabled (`.claude/skills/` for Claude Code, `.agents/skills/` for Codex).

- [isolate-refactoring-from-behavior-change](../../crates/godharness-core/skills/isolate-refactoring-from-behavior-change/SKILL.md)
- [property-based-testing](../../crates/godharness-core/skills/property-based-testing/SKILL.md)
- [atomic-commits](../../crates/godharness-core/skills/atomic-commits/SKILL.md)
- [resource-oriented-api-design](../../crates/godharness-core/skills/resource-oriented-api-design/SKILL.md)
