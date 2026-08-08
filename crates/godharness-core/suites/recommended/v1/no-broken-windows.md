---
id: no-broken-windows
title: No Broken Windows
keywords: [broken window, rotting code, decay, technical debt, neglected, degraded]
paths: []
must-read: false
supersedes: []
relates-to: [simplify-before-done]
---

## Rule

Fix a bad design, wrong decision, or degraded piece of code the moment you notice it, rather
than letting it normalize as "how this codebase is." This covers neighboring code you didn't
touch but noticed rotting, not your own work in progress.

## Why

Visible neglect changes what looks acceptable. One un-fixed broken window makes the next one
easier to ignore, and a codebase that already looks abandoned in places invites more of the
same. Fixing it the moment it's noticed keeps that signal from ever taking hold.

## How to apply

When you notice something wrong while working nearby - a stale comment, a workaround that
outlived its reason, a name that no longer matches what the code does - fix it if it's small
enough to do safely in the same change. If it's too large to fix inline, at minimum flag it
explicitly (an issue, a TODO with an owner) rather than walking past it silently.
