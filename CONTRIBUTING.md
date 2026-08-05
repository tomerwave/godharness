# Contributing to Godharness

Godharness is pre-alpha: it ships a Rust workspace scaffold and a stub CLI surface, not yet
a resolver. Real standard-schema proposals, resolver design feedback, and adapter design
feedback are as useful as code right now.

## Before opening an issue or pull request

- Search existing issues and discussions first.
- For a proposed change to the suite schema or `recommended@1` content, state which existing
  `godharness.yaml` files it would break, if any, and why the change is universal enough for
  the default suite rather than repository-specific standards.
- Discuss substantial design changes before implementing them, so the public configuration
  and `context` JSON contracts stay coherent.
- For a security issue, follow [SECURITY.md](SECURITY.md) instead of opening a public issue.

## Design principles

- Deterministic resolution: no LLM decides which context is mandatory.
- Local-first: repository source and resolved context stay local by default.
- Markdown and Git native: standards and decisions remain reviewable and portable.
- Explainability: a user can always answer "why was this injected?"
- Versioned suites: `recommended@1` stays stable when future suites change.
- Don't invent engineering standards. Godharness ships a resolver, a schema, and a small
  default suite — it does not manufacture opinions about what a team should believe.
- The `context` command's JSON output is a stable contract every adapter depends on.
  Changing its shape is a breaking change and needs to be called out as one.

## Change conventions

- Keep changes focused, reviewable, and reversible.
- Keep test code out of `src/`: crate contracts live in `crates/<crate>/tests/`.
- Update documentation when public behavior, configuration, suite defaults, or the
  `context` JSON contract changes.
- Branch from `main` and name the branch with a Conventional Commits type, a slash, and a
  lower-case description — `feat/suite-precedence`. Accepted types: `feat`, `fix`, `perf`,
  `docs`, `style`, `refactor`, `test`, `build`, `ci`, `chore`, `revert`, `release`. This
  convention is documented guidance for now; it is not yet CI-enforced — see
  [tomerwave/godlint#282](https://github.com/tomerwave/godlint/issues/282), which proposes
  the check as a shared godlint rule instead of a script duplicated across repositories.

## Pull requests

Pick the template that matches the change by appending `?template=standard-proposal.md` or
`?template=infrastructure.md` to the pull request URL: `standard-proposal` for changes to
the suite schema, `recommended@1` content, or resolver/precedence behavior; `infrastructure`
for build, CI, tooling, or documentation work.

Most labels are applied for you from the paths a pull request touches — `documentation`,
`core`, `cli`, `adapter`, `infrastructure`. Nothing to do.

## Code of Conduct

By participating, you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
