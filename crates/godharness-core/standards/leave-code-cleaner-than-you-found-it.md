---
id: leave-code-cleaner-than-you-found-it
title: Leave Code Cleaner Than You Found It
keywords: [boy scout rule, leave it cleaner, out of scope, drive-by fix]
paths: []
must-read: false
supersedes: []
relates-to: [simplify-before-done, small-reviewable-changes]
---

## Rule

When you notice a genuine defect adjacent to your change - dead code, a misleading name, a
duplicated helper, a fix you're confident is correct - don't silently skip it just because it's
outside the diff you set out to write. Either fix it or name it; never leave known-bad code
untouched purely because "that's not what I'm here for."

## Why

"Out of scope" quietly became the reason nothing ever gets better: everyone can see the same
problem and everyone defers it to someone else's diff, so it outlives every individual change
that walked past it. A review pass that finds a real issue and reports "skipped: out of scope"
without saying so anywhere durable is worse than not looking - it manufactures false confidence
that the code was checked.

## How to apply

If the fix is small and safely separable from the change's behavior, make it as part of the same
pass. If it's large enough to need its own review, don't drop it - call it out explicitly (a
tracked issue, a comment in the PR description, a line in the change's summary) so the finding
survives past the moment you noticed it. A review skill that finds true issues and reports zero
findings because everything was "in scope but not touched" has failed at its job.
