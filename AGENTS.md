# Godharness agent guide

This file is the repository navigation index for coding agents and contributors. Read the
linked documents relevant to the task before changing code or configuration.

## Project documents

- [Documentation index](docs/README.md) — everything else in `docs/`.
- [Local development](docs/local-development.md) — the build and the checks CI runs.
- [Contributing](CONTRIBUTING.md) — change conventions, branch naming, pull request templates.

The current README is a working product brief, not final product documentation: it separates
what's decided (with a pointer to the implementation) from what's still an open decision —
currently just adapter protocol/installation and the Godlint-to-Godharness rationale-linking
contract. Don't treat anything in this repository as settled just because it's written down —
check whether it's marked a decision, a stub, or a placeholder before relying on it.

## Operating rules

- Deterministic resolution only: no LLM decides which context is mandatory. This applies to
  the resolver itself, not to this file's own guidance.
- Local-first by default: repository content must never leave the machine without explicit
  user action.
- `godharness-cli context`'s JSON output (`{id, path, rule, relates-to}` per matched standard)
  is a stable contract every adapter depends on. Changing its shape is a breaking change and
  must be called out as one.
- Adapter directories (`adapters/claude-code/`, `adapters/codex/`, `adapters/pi/`) currently
  hold only a README stating their install surface and their contract with `context`. Don't
  add adapter logic without first reading the Viewstone harness study this repository's design
  docs reference — that study, not invention here, is what should decide the real
  architecture.
- Don't invent engineering standards or resolver behavior to fill a gap. If the README lists
  something as an open decision, it stays open until a design doc resolves it.
- Keep test code out of `src/`: crate contracts live in `crates/<crate>/tests/`.
- Do not commit `docs/superpowers/`; it holds brainstorming/planning working notes and is
  git-ignored.

## Current implementation status

`init`, `check`, `context`, and `doctor` are real: they load `godharness.yaml` (or defaults),
build the embedded `recommended@1` suite plus any repository-specific standards into a
supersedes/relates-to graph, and validate or resolve against it. The frontmatter schema,
keyword/path matching, and precedence rules (must-read always matches, supersedes suppresses,
relates-to surfaces neighbors) are implemented and tested — see `crates/godharness-core/tests/`
for the current behavior, not this paragraph, if a specific claim here and the code disagree.

Not implemented: every adapter (`adapters/claude-code/`, `adapters/codex/`, `adapters/pi/`
still hold only a README) and the Godlint-to-Godharness rationale-linking contract.

Whenever a PR changes what this paragraph claims is done or not done, update this paragraph in
the same PR — `scripts/check-docs-freshness.sh` (wired into CI) blocks specific superseded
claims from silently reappearing, but it can't verify new drift it doesn't know about yet.
