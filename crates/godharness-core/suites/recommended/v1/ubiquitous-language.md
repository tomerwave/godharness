---
id: ubiquitous-language
title: Ubiquitous Language
keywords: [domain language, business term, vocabulary, ubiquitous language, domain model]
paths: []
must-read: false
supersedes: []
relates-to: [naming]
---

## Rule

Use the domain expert's own words in code, tests, and conversation. Don't translate a business
term into a technical synonym just because it sounds more like software.

## Why

Every translation is a place meaning can drift: the code says "account," the business says
"customer," and eventually nobody's sure whether they mean the same thing. Naming already
covers *how* to pick a clear name generically; this is specifically about *where the word comes
from* - the domain, not an engineer's preferred synonym for it.

## How to apply

When naming a type, function, or variable that represents a domain concept, use the term a
domain expert or product spec already uses for it, even if a more "technical-sounding" word
exists. If the team's vocabulary for something is inconsistent, that's worth resolving before
picking a name, not papering over with a new synonym.
