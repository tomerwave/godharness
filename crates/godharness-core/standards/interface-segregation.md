---
id: interface-segregation
title: Interface Segregation
keywords: [interface, fat interface, unused method, segregation, contract]
paths: []
must-read: false
supersedes: []
relates-to: [design-for-extension]
---

## Rule

Don't force a caller to depend on methods or fields it never uses. Split a fat interface along
the seams different callers actually need, rather than handing everyone the same wide contract.

## Why

A caller that depends on an interface it barely uses breaks whenever any part of that
interface changes, even the parts it never touches. Narrow, purpose-built interfaces mean a
change to one caller's needs doesn't ripple into unrelated callers who happened to share the
same wide contract.

## How to apply

Before adding a method to a shared interface for one caller's benefit, check whether other
implementers or callers actually need it. If not, either add it to a smaller, more specific
interface, or give that one caller its own narrower contract instead of widening the shared one.
