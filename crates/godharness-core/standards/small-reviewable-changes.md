---
id: small-reviewable-changes
title: Small, Reviewable Changes
keywords: [large pr, big diff, small pr, split change, reviewable change, bundle changes]
paths: []
must-read: false
supersedes: []
relates-to: [architecture-decisions, small-focused-units]
---

## Rule

Keep each change small enough for a reviewer to hold in their head in one sitting - one logical
change, not a bundle of unrelated ones stitched together because they happened at the same time.

## Why

A reviewer's ability to actually catch a problem drops sharply once a diff grows past what they
can hold in working memory - past that point review becomes skimming, not verification. A large
change also hides its own risk: an unrelated refactor bundled with a feature makes it unclear
which part introduced a regression when one shows up later.

## How to apply

Before opening a PR, check whether it does one describable thing. If it bundles a refactor with
a feature, or two unrelated fixes, split it into separate PRs (or at minimum separate commits
per `atomic-commits`) so each can be reviewed and reverted independently.
