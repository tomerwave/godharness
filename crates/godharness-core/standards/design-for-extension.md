---
id: design-for-extension
title: Design for Extension, Don't Build It Yet
keywords: [extensible, reusable, future use case, plugin, over-engineering, generic]
paths: []
must-read: false
supersedes: []
relates-to: []
---

## Rule

When shaping a module's interface, ask whether it's likely to need extension later - if unclear,
ask rather than guess. If extension is likely, shape the boundary so it stays cheap later. Don't
build the extension, a plugin system, or config options for cases with no concrete evidence
they're coming.

## Why

A clean interface costs little to design up front but is expensive to retrofit once callers
depend on a rigid shape. But unused abstraction layers built "just in case" add complexity that
may never pay off - the same over-engineering trap as building unrequested features. The skill
is designing the seam, not building both sides of it, and asking is cheaper than guessing wrong
in either direction.

## How to apply

If it's unclear whether this will need to grow later, ask directly rather than guess. If a
near-future case is confirmed likely, shape today's interface so handling it later is a small
change rather than a rewrite, without building that case now. Don't add config options, plugin
hooks, or generic abstraction layers for extensions with no concrete evidence they're coming;
multiple near-term callers or an already-repeated pattern are signals worth the extra design
thought even without asking.
