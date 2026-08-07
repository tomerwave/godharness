# Changelog

All notable changes to godharness will be documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
[Semantic Versioning](https://semver.org/spec/v2.0.0.html). Before `1.0`, a `0.x` release may
change the CLI surface or configuration schema.

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
