---
id: dependency-direction
title: Dependency Direction
keywords: [dependency, import, layering, dependency inversion, inward, outward]
paths: []
must-read: false
supersedes: []
relates-to: [design-for-extension, architecture-decisions]
---

## Rule

Source-code dependencies must point inward, toward policy and business logic, never outward
toward mechanism or infrastructure detail. A domain type should not import a database driver,
an HTTP client, or a specific framework.

## Why

Code that depends outward is coupled to the volatile detail it names - swap the database and
every importer breaks. Code that only depends inward stays stable while the outer layers change
around it. This is what actually makes a "layered" architecture layered, rather than a naming
convention on top of a tangle of imports pointing every direction.

## How to apply

Before adding an import, ask which direction it points: toward a more stable abstraction, or
toward a more volatile implementation detail. If a domain or business-logic module needs to
call out to infrastructure, invert it - define an interface the domain owns, and have the
infrastructure implement it, not the other way around.
