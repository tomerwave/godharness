# Godharness agent guide

This file is the repository navigation index for coding agents and contributors. Read the
linked documents relevant to the task before changing code or configuration.

## Project documents

- [Documentation index](docs/README.md) — everything else in `docs/`.
- [Local development](docs/local-development.md) — the build and the checks CI runs.
- [Contributing](CONTRIBUTING.md) — change conventions, branch naming, pull request templates.

The current README is a working product brief, not final product documentation: the standard
schema, resolver precedence, and adapter architecture are all listed there as open decisions.
Don't treat anything in this repository as settled just because it's written down — check
whether it's marked a decision, a stub, or a placeholder before relying on it.

## Operating rules

- Deterministic resolution only: no LLM decides which context is mandatory. This applies to
  the resolver itself, not to this file's own guidance.
- Local-first by default: repository content must never leave the machine without explicit
  user action.
- `godharness-cli context`'s JSON output is a stable contract every adapter depends on.
  Changing its shape — even while it's always an empty array today — is a breaking change and
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

The workspace, a stub CLI (`init`/`check`/`context`/`doctor`, none of them doing real work
yet), and CI (test/lint/format/docs/godlint/labeler) exist. The resolver, standard schema,
selector/precedence rules, and every adapter are not implemented. `godharness-core::Config`
parses the illustrative `godharness.yaml` shape from the README but nothing consumes it yet
beyond `check`'s stub.
