# Skills

The 31 skills godharness installs as part of `recommended@1` are real, spec-compliant
`SKILL.md` files under `crates/godharness-core/skills/` — that directory is the source of
truth, not this one. `godharness adapters enable`/`update` write them into whichever tool a
repo has enabled (`.claude/skills/` for Claude Code, `.agents/skills/` for Codex).

- [isolate-refactoring-from-behavior-change](../../crates/godharness-core/skills/isolate-refactoring-from-behavior-change/SKILL.md)
- [property-based-testing](../../crates/godharness-core/skills/property-based-testing/SKILL.md)
- [atomic-commits](../../crates/godharness-core/skills/atomic-commits/SKILL.md)
- [resource-oriented-api-design](../../crates/godharness-core/skills/resource-oriented-api-design/SKILL.md)
- [systematic-debugging](../../crates/godharness-core/skills/systematic-debugging/SKILL.md)
- [verification-before-completion](../../crates/godharness-core/skills/verification-before-completion/SKILL.md)
- [requesting-code-review](../../crates/godharness-core/skills/requesting-code-review/SKILL.md)
- [receiving-code-review](../../crates/godharness-core/skills/receiving-code-review/SKILL.md)
- [simplify](../../crates/godharness-core/skills/simplify/SKILL.md)
- [ai-slop-cleaner](../../crates/godharness-core/skills/ai-slop-cleaner/SKILL.md)
- [research-with-evidence](../../crates/godharness-core/skills/research-with-evidence/SKILL.md)
- [frontend-design](../../crates/godharness-core/skills/frontend-design/SKILL.md)
- [retrospective-workflow-review](../../crates/godharness-core/skills/retrospective-workflow-review/SKILL.md)
- [clarify-before-building](../../crates/godharness-core/skills/clarify-before-building/SKILL.md)
- [doubt-driven-development](../../crates/godharness-core/skills/doubt-driven-development/SKILL.md)
- [define-goal](../../crates/godharness-core/skills/define-goal/SKILL.md)

## `refactor-ux` — gated UI/UX refactor chain

A 12-step chain for putting a fast-built, existing product through a real UI/UX pass and
ICP-aligned user journeys. Only step 1 auto-triggers on relevant prompts; each later step is
reached by finishing the previous step's exit gate and being invoked by name.

- [refactor-ux-01-frame-session](../../crates/godharness-core/skills/refactor-ux-01-frame-session/SKILL.md)
- [refactor-ux-02-confirm-goal](../../crates/godharness-core/skills/refactor-ux-02-confirm-goal/SKILL.md)
- [refactor-ux-03-draft-journeys](../../crates/godharness-core/skills/refactor-ux-03-draft-journeys/SKILL.md)
- [refactor-ux-04-icp-panel-journeys](../../crates/godharness-core/skills/refactor-ux-04-icp-panel-journeys/SKILL.md)
- [refactor-ux-05-color-scheme](../../crates/godharness-core/skills/refactor-ux-05-color-scheme/SKILL.md)
- [refactor-ux-06-mock-iterate](../../crates/godharness-core/skills/refactor-ux-06-mock-iterate/SKILL.md)
- [refactor-ux-07-consolidate](../../crates/godharness-core/skills/refactor-ux-07-consolidate/SKILL.md)
- [refactor-ux-08-framework-doctor](../../crates/godharness-core/skills/refactor-ux-08-framework-doctor/SKILL.md)
- [refactor-ux-09-build](../../crates/godharness-core/skills/refactor-ux-09-build/SKILL.md)
- [refactor-ux-10-icp-panel-look](../../crates/godharness-core/skills/refactor-ux-10-icp-panel-look/SKILL.md)
- [refactor-ux-11-final-pass](../../crates/godharness-core/skills/refactor-ux-11-final-pass/SKILL.md)
- [refactor-ux-12-finalize-docs](../../crates/godharness-core/skills/refactor-ux-12-finalize-docs/SKILL.md)

## `design-new` — gated new/early-stage design chain

A 3-step chain for setting a design direction before code accumulates around an undecided
one. Routes to/from `refactor-ux` for a product that later needs a full refactor.

- [design-new-01-design-doc](../../crates/godharness-core/skills/design-new-01-design-doc/SKILL.md)
- [design-new-02-frontend-design](../../crates/godharness-core/skills/design-new-02-frontend-design/SKILL.md)
- [design-new-03-icp-panel-look](../../crates/godharness-core/skills/design-new-03-icp-panel-look/SKILL.md)
