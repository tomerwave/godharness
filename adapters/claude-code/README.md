# Claude Code adapter (placeholder)

Not implemented. This document states the intended contract and install surface so the shape
is visible in the repository layout; implementation is deferred pending the Viewstone harness
study referenced in this repository's design docs (Viewstone's `.claude/hooks/` is a working
reference implementation of the same idea, using a TypeScript matcher instead of this core).

## Install surface

A Claude Code plugin, installable from a marketplace the way [oh-my-claudecode] and other
plugins are today (`/plugin install godharness` or equivalent). The plugin registers hooks —
expected to be `UserPromptSubmit` (prompt-level matching) and `PreToolUse` for `Edit`/`Write`
(file-path-level matching) — rather than requiring a person to run any `godharness` command by
hand.

## Contract with the core

Each hook shells out to `godharness context`:

- `UserPromptSubmit` calls `godharness context --prompt "<the user's prompt text>"`.
- `PreToolUse` (on `Edit`/`Write`) calls `godharness context --paths <the file being edited>`.

Both return a JSON array on stdout: a list of matched standards, or `[]` when nothing matches.
The hook's job is to inject that result as a `<system-reminder>` (or equivalent); it must not
compute matching itself — that logic lives once, in the core binary.

[oh-my-claudecode]: https://github.com/anthropics/claude-code
