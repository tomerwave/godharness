# Security policy

## Supported versions

Godharness is pre-alpha and no release is currently supported. Security fixes will be
published for the latest released version once releases begin.

## Reporting a vulnerability

Please do not open a public issue for a suspected vulnerability.

Use GitHub's private vulnerability reporting for this repository when it is enabled.
If private reporting is unavailable, contact the maintainers through the email address
listed in the repository profile and include `Godharness security report` in the
subject.

Please provide a clear description, reproduction steps or proof of concept, impact,
and any suggested mitigation. We will acknowledge receipt within seven days and share
status updates as the investigation progresses.

## Security boundaries

Godharness resolves and injects repository Markdown into agent context; it does not
analyze or transmit source code. Its security boundaries include:

- **Local-first resolution.** Standards, decisions, and resolved context stay on the
  local machine by default. Repository content must never leave the machine without
  explicit user action.
- **The `context` contract.** `godharness-cli context` is the interface every agent
  adapter depends on. A change that lets it emit content the caller did not request
  (arbitrary file contents, environment values, or paths outside the repository) is a
  security bug, not a feature.
- **Adapter install surfaces.** Each agent integration (Claude Code plugin, Codex
  configuration, Pi extension) runs with that tool's permissions and wires hooks that
  shell out to the `godharness` binary. A vulnerable adapter can misroute or over-share
  resolved context even if the core binary is correct; adapter changes are in scope for
  security review.
- **Configuration parsing.** `godharness.yaml` is untrusted input from a cloned
  repository. Parsing it must not execute arbitrary code, read arbitrary paths outside
  the repository, or panic in a way that stops CI silently passing.
