# Changelog

All notable changes to godharness will be documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). Before `1.0`, a `0.x` release may
change the CLI surface or configuration schema.

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
