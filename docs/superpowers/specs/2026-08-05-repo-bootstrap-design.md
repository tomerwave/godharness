# Repo Bootstrap: Skeleton, CI, and Community Health — Design

Date: 2026-08-05
Status: Approved

## Purpose

Godharness is currently greenfield: `README.md`, `LICENSE`, `.gitignore` only. This
design scaffolds the repository to the standard the sibling project Godlint holds
itself to — a Rust workspace, a CI bar that dogfoods Godlint against itself, and a
full open-source community-health suite — without inventing product behavior
(the standard schema, resolver, precedence rules) that the README already lists as
an open decision.

A separate, later design will explore Viewstone's existing `.harness/` /
`.claude/hooks` / `.codex/hooks.json` / `.pi/extensions` implementation to derive
godharness's real adapter architecture and roadmap. This design deliberately stops
short of that: adapters get placeholder directories and stated contracts, not logic.

## Scope decisions from brainstorming

- Bootstrap now: Cargo workspace + stub CLI + CI + community files, all as real,
  compiling, checkable artifacts — not placeholders that do nothing.
- Godlint wired into CI as the published GitHub Action (`tomerwave/godlint@v1`),
  not built from source — godharness is a consumer of Godlint, not a fork of its
  build.
- CI adopts godlint's "core bar" (test/lint/format/docs/dogfood) plus branch-name
  enforcement, but not coverage/mutation/real-world-corpus workflows or the
  mechanical PR-invariant script — those assume a mature rule/resolver engine and
  established conventions (e.g. a CHANGELOG-per-source-change rule) that don't
  exist yet.
- The CLI keeps the four commands from the README (`init`, `check`, `context`,
  `doctor`), reframed by audience: `context` is the adapter-facing hot-path JSON
  contract (not typed by hand); `check`/`doctor`/`init` are human/CI-facing.
- Human/agent "installation" happens per-tool (Claude Code plugin marketplace,
  Codex config, Pi extension) — each wires hooks that shell out to the
  `godharness-cli` binary. This mirrors Viewstone's per-tool adapter pattern
  exactly, with a Rust core replacing the TS matcher.
- Community-health files are adapted to godharness's actual domain (suites,
  standards, resolver, adapters) — not godlint's rule-engine text with words
  swapped.

## Repository layout

```
godharness/
├── Cargo.toml                       workspace root
├── rust-toolchain.toml              channel "1.97.1", components [clippy, rustfmt]
├── godlint.yaml                     version: 1, suites: [recommended@1]
├── AGENTS.md                        navigation index for agents
├── CLAUDE.md                        pointer -> AGENTS.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── crates/
│   ├── godharness-core/             lib: config model stub, resolver/validator stubs
│   └── godharness-cli/              bin: clap CLI, init/check/context/doctor stubs
├── adapters/
│   ├── claude-code/                 README stub: contract + Claude Code plugin install surface
│   ├── codex/                       README stub: contract + Codex hooks.json install surface
│   └── pi/                          README stub: contract + Pi extension install surface
├── docs/
│   ├── README.md                    doc index (near-empty; schema is an open decision)
│   └── local-development.md         build instructions + "checks CI runs"
├── scripts/
│   └── check-branch-name.sh
└── .github/
    ├── workflows/
    │   ├── test.yml
    │   ├── lint.yml
    │   ├── format.yml
    │   ├── docs.yml
    │   ├── godlint.yml
    │   ├── labeler.yml
    │   └── pull-request.yml         branch-name job only
    ├── labeler.yml
    ├── PULL_REQUEST_TEMPLATE/
    │   ├── standard-proposal.md
    │   └── infrastructure.md
    └── ISSUE_TEMPLATE/
        ├── bug_report.yml
        ├── feature_request.yml
        ├── research.yml
        ├── tech_debt.yml
        ├── standard_proposal.yml
        └── config.yml
```

## Crates

- `godharness-core`: library crate. Starts with a stub `Config` struct capable of
  parsing the illustrative `godharness.yaml` shape from the README (`version`,
  `suites`, `standards`, `adapters`) and a stub `resolve()` returning an empty
  result. No precedence, selector, or conflict logic yet — those are open product
  decisions, not infrastructure.
- `godharness-cli`: binary crate, `clap` derive API, four subcommands:
  - `check` — human/CI-facing. Validates config schema (via `godharness-core`).
    Stub body: parses config if present, prints "not yet implemented" for the
    rest, exits 0.
  - `doctor` — human/CI-facing. Stub: prints "not yet implemented", exits 0.
  - `init` — human-facing, one-time. Stub: prints "not yet implemented", exits 0.
  - `context --prompt <STR>` / `context --paths <GLOB>...` — adapter-facing.
    Stub: prints an empty JSON array (`[]`) to `stdout` — this establishes the
    contract shape (JSON on stdout, empty when nothing matches) that adapters and
    later resolver work must preserve.
  - Each subcommand gets one integration test in `crates/godharness-cli/tests/`
    asserting exit code 0 and, for `context`, that stdout parses as JSON.

## Adapters (placeholders only)

Each `adapters/<tool>/README.md` states, but does not implement:
- The install surface for that tool (Claude Code: plugin marketplace entry,
  `/plugin install godharness`-equivalent; Codex: `hooks.json` entry; Pi: extension
  registration).
- The contract it depends on: `godharness-cli context --prompt "..."` /
  `--paths ...` returns a JSON array of matched standards on stdout; empty array
  when nothing matches; adapter injects the result, doesn't compute matching
  itself.
- Explicit note: "Implementation deferred pending the Viewstone harness study
  (see docs/superpowers/specs for the follow-up design)."

## CI workflows

All workflows: `permissions: contents: read` (`labeler.yml` additionally needs
`pull-requests: write`), triggers `pull_request` + `push: branches: [main]` +
`workflow_dispatch`, Rust pinned via `rustup toolchain install 1.97.1 --profile
minimal` where a toolchain is needed.

| Workflow | Job |
| --- | --- |
| `test.yml` | `cargo test --workspace` |
| `lint.yml` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `format.yml` | `cargo fmt --all -- --check` |
| `docs.yml` | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` |
| `godlint.yml` | `uses: tomerwave/godlint@v1` against the repo root |
| `labeler.yml` | `actions/labeler@v5`, `sync-labels: false`, on `pull_request_target` |
| `pull-request.yml` | branch-name job only: `scripts/check-branch-name.sh` against `$BRANCH` |

`.github/labeler.yml` path rules:
- `documentation` → `docs/**`, `*.md`
- `core` → `crates/godharness-core/**`
- `cli` → `crates/godharness-cli/**`
- `adapter` → `adapters/**`
- `infrastructure` → `.github/**`, `Cargo.toml`, `crates/*/Cargo.toml`, `rust-toolchain.toml`

**Deferred, not included in this bootstrap:** coverage-threshold workflow,
mutation-testing workflow, real-world-corpus workflow, mechanical PR-invariant
script (`validate-pull-request.py` equivalent), CHANGELOG-entry-per-source-change
enforcement. Each assumes conventions (a rule/resolver engine to measure, an
established changelog discipline) that don't exist yet in this repo.

`scripts/check-branch-name.sh` enforces the same convention as godlint: a
Conventional Commits type (`feat`, `fix`, `perf`, `docs`, `style`, `refactor`,
`test`, `build`, `ci`, `chore`, `revert`, `release`), a slash, and a lower-case
slug, e.g. `feat/context-json-contract`.

## Community health files

- **CODE_OF_CONDUCT.md** — Contributor Covenant, copied from godlint verbatim (no
  project-specific content to adapt).
- **SECURITY.md** — same reporting process shape as godlint's (private
  vulnerability reporting, seven-day acknowledgment), but godharness-specific
  security boundary: godharness resolves and injects repository Markdown rather
  than analyzing source, so the stated boundary is local-first resolution (no
  repository content leaves the machine without explicit user action) and the
  per-tool adapter surface (each agent hook/extension) as an attack surface.
- **CONTRIBUTING.md** — godharness-specific design principles (deterministic
  resolution, local-first, Markdown/Git-native, explainability, versioned suite
  stability, "don't invent engineering standards — godharness ships a
  resolver/schema, not opinions"), the branch-naming convention, and PR template
  selection (`?template=standard-proposal.md` / `?template=infrastructure.md`).
  No `fixes-false-positive`/`relaxes-a-rule` labels — that concept is specific to
  godlint's release-drift dogfooding and doesn't apply here.
- **PR templates**:
  - `standard-proposal.md` — for changes to the suite schema, `recommended@1`
    content, or resolver/precedence behavior: what changes, why it belongs in the
    universal default suite (vs. resolver logic), backward compatibility for
    existing `godharness.yaml` files, validation performed.
  - `infrastructure.md` — CI/build/tooling/docs changes: summary, effect on
    contributors, validation, checklist (no schema/behavior change without the
    other template; docs describe what exists today).
- **Issue templates** — `bug_report.yml`, `feature_request.yml`, `research.yml`,
  `tech_debt.yml`, `config.yml` adapted from godlint's with godharness-specific
  wording (standards/suites/resolver/adapters instead of rules/analyzers).
  `rule_proposal.yml` becomes `standard_proposal.yml`: propose a change to the
  default suite or schema rather than a lint rule, covering what standard/schema
  changes, why it's universal enough for `recommended@1` (or why it's resolver
  behavior instead), and compatibility impact.

All of the above get real, godharness-specific prose when written — not a
find-and-replace of godlint's text.

## AGENTS.md / CLAUDE.md / docs/local-development.md

- **AGENTS.md**: navigation index linking `docs/product-scope.md` (reorganized
  from the current README), `docs/architecture.md` (core/cli/adapter
  boundaries), `docs/local-development.md`, `CONTRIBUTING.md`. Operating rules
  specific to godharness: deterministic resolution only, local-first by default,
  `context`'s JSON output is a stable contract — changing its shape is a breaking
  change — and adapter directories state their contract in their own README
  until real logic lands. A "current implementation status" section stating
  plainly: workspace + stub CLI only, resolver/schema not implemented, adapters
  not implemented pending the Viewstone study.
- **CLAUDE.md**: one line — `See [AGENTS.md](AGENTS.md) for all project rules and
  standards.`
- **docs/local-development.md**: build instructions, the "checks CI runs" block
  mirroring godlint's (`cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace`,
  `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`), a note that
  `godlint check` runs via the Action rather than a local build, and a note on
  which commands are human-facing vs. adapter-facing.

## Explicit non-goals of this bootstrap

- No resolver, selector, precedence, or conflict-handling logic — those are the
  README's own "Open Decisions."
- No adapter implementation — placeholder READMEs only, pending the Viewstone
  harness study (a separate, later design).
- No coverage/mutation/real-world-corpus CI, no mechanical PR-invariant script.
- No release/packaging workflow (crates.io/npm/PyPI) — premature before there is
  a real binary behavior to release.

## Follow-up

A separate brainstorming/design pass will study Viewstone's `.harness/`,
`.claude/hooks/`, `.codex/hooks.json`, and `.pi/extensions/` implementation to
derive godharness's adapter architecture and a concrete roadmap, informing what
`adapters/*/README.md` currently only states as a contract.
