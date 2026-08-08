---
id: server-side-authorization
title: Server-Side Authorization
keywords: [authorization, access control, authz, permission check, deny by default]
paths: []
must-read: false
supersedes: []
relates-to: [injection-parameterize, secrets-and-security]
---

## Rule

Check that the authenticated identity is permitted to act on the specific resource on every
server-side request. Deny by default, and never rely on a hidden UI element or a client-side
check as the actual access control.

## Why

A client-side check only prevents an honest client from showing a button - it does nothing
against a request sent directly. The server is the only place a permission check can't be
bypassed, which means every server-side handler that acts on a resource needs its own check for
*this* resource, not just proof the caller is logged in.

## How to apply

For every request handler that reads or mutates a specific resource, verify the caller is
authorized for that resource - not just authenticated - before acting. Default to denying
access and require an explicit grant, rather than allowing by default and trying to enumerate
every case that should be blocked.
