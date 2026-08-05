# Codex adapter (placeholder)

Not implemented. This document states the intended contract and install surface so the shape
is visible in the repository layout; implementation is deferred pending the Viewstone harness
study referenced in this repository's design docs (Viewstone's `.codex/hooks.json` is a
working reference implementation of the same idea, using a TypeScript matcher instead of this
core).

## Install surface

A `hooks.json` entry (or the equivalent `[hooks]` table in `config.toml`) that a user adds to
their Codex configuration, registering the same event names Claude Code uses so one contract
serves both tools.

## Contract with the core

Each hook shells out to `godharness context`, exactly as the Claude Code adapter does:

- A prompt-level hook calls `godharness context --prompt "<the prompt text>"`.
- A file-edit-level hook calls `godharness context --paths <the file being edited>`.

Both return a JSON array on stdout: a list of matched standards, or `[]` when nothing matches.
The hook injects that result; it does not compute matching itself.
