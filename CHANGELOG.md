# Changelog

All notable changes to godharness will be documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). Before `1.0`, a `0.x` release may
change the CLI surface or configuration schema.

## [0.1.5] - 2026-08-09

### Added

- `recommended@1` expanded from 4 to 15 skills and from 29 to 31 standards, following a
  survey of Superpowers, Claude Code/Anthropic's official skills, oh-my-claudecode, and
  `addyosmani/agent-skills`. New skills: `systematic-debugging`, `verification-before-completion`,
  `requesting-code-review`, `receiving-code-review`, `simplify`, `ai-slop-cleaner`,
  `research-with-evidence`, `frontend-design`, `retrospective-workflow-review`,
  `clarify-before-building`, `doubt-driven-development`. New standards:
  `leave-code-cleaner-than-you-found-it` (the Boy Scout Rule) and `concise-communication`,
  both must-read.

### Internal

- Nine hand-written error types collapsed onto a shared `string_error!` macro; registry
  standard/skill parsing is now cached instead of re-parsing on every call; keyword and path
  matching cache their compiled regex/glob patterns.
- Nine duplicated test temp-directory helpers and four duplicated `Standard` test builders
  consolidated into `tests/common/mod.rs` per crate.

## [0.1.4] - 2026-08-08

### Added

- `recommended@1` now bundles 4 skills alongside its 29 standards —
  isolate-refactoring-from-behavior-change, property-based-testing, atomic-commits,
  resource-oriented-api-design — as real `SKILL.md` files. `godharness adapters
  enable`/`update` install them into `.claude/skills/` (Claude Code) or `.agents/skills/`
  (Codex), merge-not-clobber, the same idempotency model as hook config.

### Internal

- Standards and skills moved off suite/version-nested storage (`suites/recommended/v1/`)
  into a flat registry (`crates/godharness-core/src/registry.rs`) with suites as manifests
  referencing registry ids by reference, mirroring godlint's rules/suites split — a future
  second suite can reuse standards/skills with zero file duplication.
- `registry::standard()`/`skill()` now parse the embedded corpus once instead of on every
  call; `update_repository` no longer reads `godharness.yaml` twice per invocation; keyword
  and path matching cache their compiled regex/glob patterns instead of recompiling per call.
- Nine hand-written error types collapsed onto a shared macro; nine near-identical test
  temp-directory helpers and four `Standard` test builders consolidated into
  `tests/common/mod.rs` per crate.

## [0.1.3] - 2026-08-08

### Added

- `recommended@1` expanded from 26 to 29 standards: test-data-builders,
  structured-logging-over-printf, and small-reviewable-changes.

## [0.1.2] - 2026-08-08

### Added

- `recommended@1` expanded from 13 to 26 standards: dependency-direction,
  interface-segregation, liskov-substitutability, single-level-of-abstraction,
  dont-repeat-yourself, no-broken-windows, ubiquitous-language, test-pyramid-shape,
  test-independence, flaky-test-is-signal, injection-parameterize,
  server-side-authorization, and pin-and-verify-dependencies, drawn from Clean Code, Clean
  Architecture, SOLID, DDD, testing practice, and security sources.

## [0.1.1] - 2026-08-07

### Fixed

- The `homebrew` release job and the self-update commit script now detect a brand-new,
  untracked file as a real change (`git diff --quiet` is blind to untracked files, which
  silently no-op'd the very first Homebrew tap publish in 0.1.0).

### Internal

- npm publishing reverted from a bootstrap token back to OIDC trusted publishing, now that all
  six packages exist and each has trusted publishing configured.

## [0.1.0] - 2026-08-07

### Added

- Initial godharness release: standards engine, `recommended@1` suite, `init`/`check`/`context`/
  `doctor` commands, and the Claude Code/Codex live-hook adapter.
- `godharness adapters enable <claude-code|codex>` — wires the live-hook adapter into a repo's
  `.claude/settings.json` or `.codex/hooks.json`, merging rather than clobbering existing config.
- `godharness update` — resyncs suite pins and enabled-adapter hook config to the installed
  version.
- Release pipeline publishing to crates.io, npm, PyPI, and a Homebrew tap, with a self-update
  workflow that keeps this repository's own installation current automatically.
