<p align="center">
  <img src="assets/godharness-icon.svg" width="168" alt="Godharness logo">
</p>

# Godharness: Temporary Product Brief

Status: Working notes — not a Viewstone product document
Date: 2026-08-05

## Problem

Coding agents can enforce deterministic rules after changing code, but they do
not automatically know a repository's architecture decisions, product intent,
engineering practices, or the rationale behind policies. Raw documentation is
easy to miss, too broad to load in full, and difficult to keep structurally
healthy.

## Desired Outcome

Godharness gives coding agents the right engineering context before they act.
It ships a small, versioned, opinionated default suite of best practices and
lets each repository add its own standards, decision records, product context,
and service-specific guidance.

## Product Positioning

Godharness is an agent-context and documentation-governance framework. It is
related to Oh My Claude Code and Superpowers in that it supports coding-agent
workflows, but its job is distinct:

- Workflow frameworks prescribe how an agent works.
- Godharness supplies the project-specific and default context the agent needs
  to make sound decisions.
- Godlint deterministically verifies the codifiable subset in code and CI.

The family promise is:

> Godharness helps agents understand the project. Godlint makes sure the
> project stays true to what it decided.

## Relationship to Godlint

Godlint and Godharness have deliberately different authority boundaries.

| Product | Authority | Example |
| --- | --- | --- |
| Godharness | Guidance before a change | "Validate untrusted data at runtime boundaries." |
| Godlint | Deterministic enforcement after a change | "Do not read environment variables outside the configured boundary." |

Godharness handles decisions that cannot safely be inferred from syntax:

- product intent and non-goals;
- architecture tradeoffs and ADRs;
- testing philosophy and recovery strategy;
- domain vocabulary and service ownership;
- instructions for evaluating ambiguous cases.

Godlint handles objective, explainable checks against source and workflows.
Godharness must not claim that injecting guidance proves compliance; Godlint,
tests, review, and other deterministic checks remain the proof mechanisms.

## Version 1 Scope

Godharness v1 is a local-first, installable CLI with Markdown and Git as the
source of truth. It has no user interface and no Obsidian integration in v1.

### Recommended suite

`recommended@1` ships a small, curated, versioned set of broadly useful
standards, such as:

- testing and verification;
- error handling;
- small focused units;
- security and secrets hygiene;
- configuration boundaries;
- CI expectations;
- documentation and decision-record practices.

The suite is not injected wholesale. The resolver selects a tiny universal
core plus standards relevant to the task and changed paths. Suites must remain
small enough that their context is useful rather than generic noise.

### Repository context

Repositories compose the default suite with their own Markdown standards,
architecture decisions, product context, playbooks, and service-specific
guidance. Project content can refine the default guidance without copying it.

Illustrative configuration:

```yaml
version: 1
suites: [recommended@1]

standards:
  - docs/engineering/**
  - docs/process/**
  - docs/architecture/**

adapters:
  codex: true
  claude-code: true
  pi: true
```

### Commands

The initial CLI surface should be small:

```text
godharness init
godharness check
godharness context --prompt "..."
godharness context --paths services/api/src/auth.ts
godharness doctor
```

- `init` creates the configuration and starter standards.
- `check` validates document schema, links, classification, selectors, and
  adapter configuration.
- `context` resolves the standards and decisions relevant to a prompt or set
  of changed paths in a deterministic machine-readable form.
- `doctor` validates local installation and adapter wiring.

## Technical Direction

Build the core in Rust.

Reasons:

- a single fast binary works locally and in CI without Node or Bun;
- the resolver, validator, configuration model, diagnostics, and release
  approach align with Godlint;
- it supports deterministic, local-first behavior and cross-platform installs;
- thin agent-specific adapters can call the binary and consume JSON rather than
  duplicating matching logic.

The existing Viewstone TypeScript harness is a useful reference implementation,
not the product core. It proves three useful mechanisms:

- prompt-keyword matching;
- changed-path glob matching;
- adapters for Codex, Claude Code, and Pi that inject selected context.

The extracted core should use a real YAML/frontmatter parser and schema
validation rather than preserving the current hand-written parser.

## Required Product Properties

- Deterministic resolution: no LLM decides which context is mandatory.
- Local-first: repository source and context stay local by default.
- Markdown and Git native: standards and decisions remain reviewable and
  portable.
- Explainability: users can answer "why was this injected?".
- Versioned suites: `recommended@1` remains stable when future suites change.
- Scoped context: support universal, task, path, service, and repository scope.
- Conflict handling: define precedence and surface contradictory standards.
- CI validation: prevent broken links, malformed metadata, unindexed
  standards, stale ownership, and adapter drift.

## Explicit Non-Goals for v1

- No graphical interface, desktop application, or Obsidian integration.
- No generic notes app or knowledge graph.
- No vector search/RAG as the policy-selection mechanism.
- No LLM-driven enforcement or opaque context selection.
- No duplicate source-code linter; Godlint owns deterministic source and
  workflow policy.
- No automatic invention of engineering standards.

## Later Possibilities

An Obsidian-like product experience could be valuable later for authoring,
exploring relationships, ownership/review dates, contradictions, and context
previews. It must remain a view over the Git-backed Markdown source of truth,
not replace it.

Future Godlint integration can map a finding to its relevant Godharness
standard so developers receive both the deterministic violation and the
decision rationale.

## Risks

- A large default suite will become generic prompt noise and be ignored.
- Simple keyword and glob matching can over-match or miss relevant context.
- Injecting a rule does not prove an agent followed it.
- Project-specific standards can conflict unless precedence and ownership are
  explicit.
- A UI too early would distract from the core resolution and validation loop.

## Open Decisions

- Exact standard frontmatter schema and selectors.
- Priority, precedence, and conflict-reporting rules.
- How a project extends or overrides a recommended standard.
- The minimal universal core that every task receives.
- Adapter protocol and installation model for each supported agent.
- The first Godlint-to-Godharness rationale-linking contract.

