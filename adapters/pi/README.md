# Pi adapter (placeholder)

Not implemented. This document states the intended contract and install surface so the shape
is visible in the repository layout; implementation is deferred pending the Viewstone harness
study referenced in this repository's design docs (Viewstone's `.pi/extensions/` is a working
reference implementation of the same idea, using a TypeScript matcher instead of this core).

## Install surface

A Pi extension (`pi.on("before_agent_start" | "tool_call", handler)`), registered the way any
other Pi extension is installed, running in-process rather than as a separate hook script.

## Contract with the core

The extension shells out to `godharness context` from its handler:

- `before_agent_start` calls `godharness context --prompt "<the prompt text>"`.
- `tool_call` (on an edit-equivalent tool) calls `godharness context --paths <the file being
  edited>`.

Both return a JSON array on stdout: a list of matched standards, or `[]` when nothing matches.
The extension translates that result into Pi's `{message, systemPrompt}` /
block-or-allow return shape; it does not compute matching itself.
