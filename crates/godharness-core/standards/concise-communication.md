---
id: concise-communication
title: Concise Communication
keywords: [be concise, verbose, conciseness, terse, filler words]
paths: []
must-read: false
supersedes: []
relates-to: [structured-logging-over-printf]
---

## Rule

State the conclusion first, then the minimum support it needs. Drop hedging ("it seems",
"probably", "I think"), filler ("I'll go ahead and", "let's take a look at"), and restating the
question before answering it.

## Why

Verbosity isn't neutral - every hedge and pleasantry is a real cost paid by whoever reads the
output next, whether that's a human scanning a PR description or another agent parsing a report
for a decision. Padding doesn't make a claim more trustworthy; stating the finding plainly and
letting the evidence carry the weight does.

## How to apply

Lead each answer, report, or commit message with the result, not the process that produced it.
Cut sentences that only restate what was just asked. Prefer a direct claim over a hedged one
when you actually have the evidence to back it - hedge only when the uncertainty is real and
worth naming, not as a reflexive softener. This is about cutting words that carry no information,
not about cutting the reasoning a reader genuinely needs to trust the conclusion.
