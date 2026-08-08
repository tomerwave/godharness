# Adapter architecture

Status: partially implemented. The Claude Code and Codex hook adapters, and their skill
installation (`.claude/skills/`, `.agents/skills/`), are built, tested, and merged; everything
else below is still proposed. Nothing in this document is a decision until code and tests back
it; treat it the way AGENTS.md says to treat anything else written down here.

## Goal and scope

godharness is a tool other people install into their own repositories, not a script that
only serves this one. This repository dogfoods it — `godharness init`/`check` run against
godharness's own source the same way they'd run against any other project — but the design
has to hold for an arbitrary repo and an arbitrary user, not assume anything about this one.

Every adapter described here must ship with real tests before it counts as done. "Verified
against the docs" is not sufficient — see [Testing strategy](#testing-strategy).

## Two adapter kinds

Every coding-agent tool surveyed falls into one of two mechanisms for receiving external
context, and the two need genuinely different adapter shapes:

- **Static-file**: the tool reads a config file once (per session, or whenever it notices the
  file changed) — no dynamic per-prompt or per-edit signal. godharness's job is to *write*
  the right file(s) in the right format.
- **Live-hook**: the tool calls out to an external command (or, for one surveyed tool, an
  in-process plugin) at specific lifecycle events — prompt submitted, file about to be
  edited, session started. godharness's job is to *respond* to that call correctly and
  quickly.

A tool can need both (Claude Code does — see below). Neither kind needs to duplicate the
actual resolution logic: `godharness context` already is the shared core. A static adapter
calls it once at generation time; a live-hook adapter calls it per event. Nothing about
"which standards match" is reimplemented per adapter.

## Static-file adapters

### Two generic renderers, not one per tool

Surveyed static-file tools split into two shapes:

**Shape A — one file per standard, YAML frontmatter, glob-scoped.** Cursor
(`.cursor/rules/*.mdc`), Claude Code (`.claude/rules/*.md`), GitHub Copilot
(`.github/instructions/*.instructions.md`), Cline (`.clinerules/*.md`). These differ only in:

- the scope-key frontmatter field name (`globs`, `applyTo`, or Claude Code's undocumented
  `globs` — see [Claude Code](#claude-code) below for why "undocumented" matters),
- whether an "always include" flag exists and its name (`alwaysApply` for Cursor; no
  equivalent found for Copilot or Cline),
- the file-naming pattern and directory.

One generic renderer, parameterized by a small per-tool `FieldMapping`-shaped config (scope
key name, always-flag key name if any, file naming template), covers all four. This is a
render function from `&[Standard]` to `Vec<(PathBuf, String)>`, not four separate
implementations.

**Shape B — one concatenated file, no scoping at all.** Windsurf (`.windsurfrules`), Aider
(`CONVENTIONS.md`), Continue (numbered `NN-name.md` files). No frontmatter, no glob-matching
— the whole resolved suite gets rendered as prose. A second, simpler generic renderer
("render every standard's Rule/Why/How-to-apply as a heading, concatenate, optionally
number") covers these three.

### Keyword-only standards have nowhere to go

No static-file format found supports prompt-keyword triggering — only path-glob scoping and
an optional "always include" flag exist anywhere in the static world. A standard with
`keywords` but no `paths` has no destination in a rendered file. This has to be an explicit,
visible choice per standard (drop it from static output, or force it into the "always"
bucket) — not something a renderer silently decides.

### Regeneration, not scaffold-once

`init`'s current `write_if_absent` pattern (write a starter file, never touch it again) is
wrong for adapter output. Standard content changes over time — a rendered rule file is
*derived* data, and has to be regenerated whenever the underlying standards change, the same
way a lockfile gets regenerated rather than hand-edited. This needs its own write path,
separate from `init`'s scaffold-once semantics, likely surfaced as part of `check` or a new
`godharness adapters sync`.

### Per-tool notes

| Tool | Format | Confidence | Note |
|---|---|---|---|
| Cursor | `.cursor/rules/*.mdc`, `description`/`globs`/`alwaysApply` frontmatter | Medium — from third-party docs, not independently verified | Closest schema match to godharness's own `Standard` of anything surveyed. |
| Claude Code | `.claude/rules/*.md` | **Confirmed broken, not just risky** | See [Claude Code](#claude-code) — dropped as a target, not attempted for any other tool yet either. |
| GitHub Copilot | `.github/instructions/*.instructions.md`, `applyTo` frontmatter | Medium | Path-only, no "always" flag found. Also noted: path-scoped instructions are documented as supported for Copilot cloud agent / code review on GitHub.com — whether they apply the same way to local Copilot Chat/completions needs direct verification, not assumed from the docs. |
| Cline | `.clinerules/*.md` directory | Low — glob-scoping specifics not confirmed | Needs direct verification before relying on any scoping behavior. |
| Windsurf | `.windsurfrules`, freeform text | Medium | No scoping of any kind; whole-suite dump only. |
| Aider | `CONVENTIONS.md`, loaded via `--read`/`/read` | Medium | Whole-file, marked read-only for prompt caching. |
| Continue | Numbered `NN-name.md` files | Low | Ordering-only scoping found; no conditional logic. |

## Live-hook adapters

### Shared core, thin per-tool shim — but two structurally different shim kinds

**JSON-over-stdio tools** (Claude Code, Codex, Gemini CLI): the tool spawns a command, writes
JSON to its stdin, reads JSON from its stdout, and uses the exit code as part of the decision
protocol. The resolution logic is 100% shared — it's `godharness context` — only field names
and event names differ per tool, and for Claude Code and Codex specifically, confirmed by
real invocation, they don't even differ: the same stdin fields, the same output shape. This
reduces to one shared core with a per-tool mapping table at the I/O boundary only where a
tool's contract actually diverges, structurally identical in spirit to the static-file
`FieldMapping` approach.

Concretely: no separate generated shell script is needed. Since godharness itself is the
thing users install (see [Distribution](#distribution)), the hook config can point directly
at the `godharness` binary with an internal subcommand, e.g.
`godharness adapter-hook claude-code --event user-prompt-submit`, reading stdin and emitting
the right stdout JSON itself. One binary, nothing else to keep in sync on disk.

**In-process plugin tools** (opencode, and any future tool shaped like it): these aren't
spawned processes — opencode plugins are JS/TS modules loaded into opencode's own runtime.
godharness cannot be that plugin directly; the adapter is a separately-published, minimal
JS/TS shim whose only job is "read the plugin-API event, shell out to `godharness context`,
hand the result back through the plugin's own context-injection API." Different language,
same principle: zero decision logic in the shim.

### Config merge, not clobber

A real target repo likely already has its own `.claude/settings.json` (or equivalent) with
unrelated hooks in it. Whatever writes hook config has to add godharness's entries alongside
what's there, never overwrite the file outright.

### Per-tool findings

#### Claude Code

**Built and merged.** `godharness adapter-hook claude-code --event <user-prompt-submit|
session-start>` — `UserPromptSubmit` matches every standard (must-read or not) purely by
keyword, repeatable prompt after prompt with a configurable debounce
(`reinject-after-prompts` in `godharness.yaml`); `SessionStart` guarantees must-read
standards regardless of keyword match, and is source-independent by construction (it never
inspects the `source` field), so it re-fires after compact/resume/clear/fork with no extra
code. Verified end-to-end: a real standalone `claude -p` process, with a real
`.claude/settings.json` hook pointing at the built binary, correctly received the injected
`additionalContext`.

`.claude/rules/*.md` (a native static mechanism, not present in viewstone's research) was
tried first as a lower-risk alternative to a live hook for path-scoped standards, and
**confirmed broken, not just risky**: four independent negative tests (two blind subagents,
a real standalone `claude -p` process, both the documented `paths:` array syntax and the
corrected comma-separated `globs:` syntax from a linked GitHub issue) all found no
injection. Cross-referencing turned up three open, confirmed bugs on Anthropic's own repo
saying the same thing
([#16853](https://github.com/anthropics/claude-code/issues/16853),
[#21858](https://github.com/anthropics/claude-code/issues/21858),
[#22170](https://github.com/anthropics/claude-code/issues/22170)) — one reporter states "this
never worked." Dropped as a target entirely; the hook adapter covers must-read and
keyword-matched standards, and no path-glob-only static channel exists for Claude Code today.

`PreToolUse`'s `additionalContext` support was flagged as unresolved from docs alone before
this work started; it's moot now since `.claude/rules/` isn't being pursued and
`UserPromptSubmit`/`SessionStart` cover the two channels this project actually needs.

Distribution: Claude Code plugins can bundle `hooks/hooks.json` and be installed through the
marketplace mechanism, which is worth using once demand exists, rather than requiring users
to hand-edit `.claude/settings.json`.

**Skills, built 2026-08-08.** `godharness adapters enable claude-code`/`update` write a suite's
skills as real `SKILL.md` files under `.claude/skills/<id>/SKILL.md` — Claude Code's own native
skill format and directory, discovered by its live directory watch (no restart needed).
Verified with a real `claude -p` session explicitly confirming all 4 shipped skills
(isolate-refactoring-from-behavior-change, property-based-testing, atomic-commits,
resource-oriented-api-design) were visible and loadable.

#### Codex

**Built and merged — reuses the Claude Code adapter's code verbatim, no new logic.**
Verified with two independent real `codex exec --dangerously-bypass-hook-trust` runs (once
per CLI tool-argument name, `claude-code` then `codex`, to rule out anything argument-specific)
against a real `.codex/hooks.json` pointing at the built binary. Both times the injected
`additionalContext` was genuinely visible to the model.

The risk assessment before verification overstated the danger, worth recording so it isn't
repeated: third-party docs described Codex's `hooks.json` shape as internally contradictory
(event names at file root with no wrapper, versus various wrapper shapes). The real shape,
confirmed by reading `~/.codex/hooks.json` from this machine's own already-working
`oh-my-codex` installation (a genuine, currently-firing production config, the best ground
truth available) — **is structurally identical to Claude Code's**: the same `{"hooks": {...}}`
wrapper, the same PascalCase event names (`SessionStart`, `UserPromptSubmit`, `PreToolUse`,
`PostToolUse`, `Stop`), the same `matcher` field, and — confirmed by reading that
installation's own hook script — the same `{"hookSpecificOutput": {"hookEventName": ...,
"additionalContext": ...}}` output shape for `SessionStart`/`UserPromptSubmit`. This is why
zero new adapter code was needed: the existing Claude Code implementation's stdin/stdout
contract already matched.

One real, version-specific finding from the live run: `[features].codex_hooks` is
**deprecated**, replaced by `[features].hooks` (Codex printed the deprecation warning
directly). Config generated by a future `godharness adapters enable codex` should target the
new flag name.

Two things from the original research remain genuinely open, not resolved by this
verification, since neither was exercised: `PreToolUse` still only covers Bash plus
`apply_patch` as of the linked issue
([openai/codex#18491](https://github.com/openai/codex/issues/18491)), so a future
path-triggered Codex channel needs its own verification pass; and the repo-local-config bug
([openai/codex#17532](https://github.com/openai/codex/issues/17532)) was reported for hooks
configured inside `.codex/config.toml`, not the project-level `.codex/hooks.json` file this
adapter actually uses — untested whether that distinction matters, but the verified path here
avoided the exact configuration shape the bug report names.

Codex also reads `AGENTS.md` natively (confirmed — it's one of 28+ tools on the now-Linux-
Foundation-stewarded AGENTS.md convention, which this repository already follows), giving any
repo installing godharness a zero-risk baseline even before the hook adapter is configured.

**Skills, built 2026-08-08.** Codex adopted the same open `SKILL.md` format
([agentskills.io](https://agentskills.io)) as Claude Code, explicitly deprecating its older
`.codex/prompts/*.md` custom-prompts mechanism in favor of it — but scans a different
directory, `.agents/skills/<id>/SKILL.md`, discovered at session startup. `godharness adapters
enable codex`/`update` write to that path. Verified with a real `codex exec
--dangerously-bypass-hook-trust` session explicitly confirming all 4 shipped skills were
visible. This resolves the design doc's flagged risk for the case tested — a fresh session
after install — but Codex's *mid-session* live-reload behavior for skills remains undocumented
and unverified, since a fresh `codex exec` process necessarily rescans at its own startup
regardless.

#### Gemini CLI

Official, first-party, stable hook system — the richest event model surveyed after Claude
Code (`BeforeAgent`, `BeforeTool`, `SessionStart`, `PreCompress`, etc.), documented at
`docs/hooks/reference.md` in the `google-gemini/gemini-cli` repo. Close to Claude Code's
shape but **not identical** — the `additionalContext` stdout example found doesn't wrap in a
`hookEventName` field the way Claude Code's does. Needs its own mapping entry, not a shared
one with Claude Code, even though the pattern (shared core, thin shim) is the same.

#### opencode

Native plugin system (JS/TS, `.opencode/plugins/` or an npm package), 30+ hooks
(`tool.execute.before/after`, `file.edited`, session lifecycle events) — richer event
granularity than anything else surveyed, but the documented context-injection path is
centered on a compaction hook (`output.context.push()`), a different model from immediate
per-prompt injection. Requires the in-process-plugin adapter shape described above, and a
separate npm-published package, not a config file a user drops in. Lowest priority — see
[Build order](#build-order).

## Installation model

A repo installs an adapter through a godharness subcommand — not by hand-editing config —
that:

1. Writes/regenerates the static-file output for that tool (idempotent, safe to rerun).
2. For live-hook tools, adds godharness's hook entries into that tool's config, merging with
   whatever's already there rather than overwriting the file.
3. Assumes `godharness` is on `PATH` (true after any of the install methods in
   [Distribution](#distribution)) — hook config references the bare command name, never an
   absolute path resolved at install time.

Proposed CLI surface (naming, not final): `godharness adapters list`,
`godharness adapters enable <name>`, `godharness adapters sync` (regenerate static output +
verify hook config is still present after a manual edit), and an internal
`godharness adapter-hook <tool> --event <event>` that the generated hook config itself
invokes — not meant for a human to type.

## Distribution

Same pipeline as godlint's actual `release.yml`, verified directly rather than assumed:
cross-platform binaries (macOS x86_64/arm64, Linux x86_64/arm64 including musl, Windows)
attached to a GitHub Release, published in parallel to crates.io, npm, and PyPI, plus a
floating major-version git tag for GitHub Action consumers. Every binary's `--version` output
and, for musl targets, static linking, are verified in CI before anything ships — that
pattern carries over directly.

**Built**: `.github/workflows/release.yml`, ported directly from godlint's actual pipeline —
tag-consistency check, the seven-target binary matrix, and parallel crates.io/npm/PyPI
publish jobs, an `announce` job for the GitHub release plus a floating major tag, and a
terminal `homebrew` job publishing to the `tomerwave/homebrew-tap` repository (new; godlint
doesn't have this yet either, but has an in-progress branch this was modeled on).

**Also built, not something godlint needs**: `godharness update` and
`.github/workflows/self-update.yml` close the loop for this repository specifically, since
godharness (unlike godlint) doesn't check itself out from source in its own CI — the release
workflow's `announce` job fires a `repository_dispatch` here, and the self-update workflow
upgrades the installed binary, runs `godharness update` to resync suite pins and adapter
config, and commits directly to `main` if anything changed.

## Testing strategy

"Verified against research" is not sufficient for anything in this document — every claim
above that came from third-party docs or a single source is marked with a confidence level
for exactly that reason. Before any adapter is considered done:

1. **Unit tests** on the pure mapping/render logic — `Standard` → frontmatter, JSON-in →
   JSON-out — with no external tool involved. Fast, runs in normal CI, covers the actual
   godharness-owned logic.
2. **Integration tests** that feed the *real* stdin shape each tool sends (captured during
   verification, not guessed from docs) into `godharness adapter-hook ...` and assert the
   exact stdout. This tests the translation boundary without needing the external tool
   installed in CI.
3. **Real-invocation verification** — an actual run against the real tool (`codex exec
   --dangerously-bypass-hook-trust`, a real Claude Code session, etc.), following this
   repository's own `verify-through-real-path` standard: a claim that a hook "works" only
   counts if the test exercised the tool's real invocation path, not a hand-built proxy for
   it. Automated in CI wherever the tool supports non-interactive invocation; documented as a
   manual, repeatable step where it doesn't, rather than skipped.

## Open risks, flagged explicitly rather than assumed away

- Codex: `PreToolUse` path-triggered standards (Bash/`apply_patch` only, per
  [openai/codex#18491](https://github.com/openai/codex/issues/18491)) and whether the
  repo-local-config bug ([openai/codex#17532](https://github.com/openai/codex/issues/17532))
  affects any future configuration shape beyond the project-level `.codex/hooks.json` already
  verified — both unexercised by the verification done so far, not resolved by it.
- Cursor/Copilot/Cline: frontmatter field behavior sourced from third-party docs only, not
  independently confirmed the way the Claude Code `.claude/rules/` bug was — and given that
  bug turned out to be real, these should be verified the same way before being relied on.
- Every "confidence: medium/low" row in the static-file table above is real future
  verification work, not a settled fact this document is asserting.

## Build order

Ranked by risk and dependency, with Claude Code and Codex prioritized per direct instruction.
Struck-through steps are done.

1. ~~**Generic static-file renderer** (Shape A)~~ — built as reusable infrastructure
   (`render_shape_a`/`FieldMapping`/`write_rendered_files`), but its original target,
   Claude Code's `.claude/rules/*.md`, was dropped once confirmed broken (see
   [Claude Code](#claude-code)). No per-tool mapping ships yet — every "confidence" row in
   the static-file table is still unverified.
2. ~~**Claude Code hook adapter**~~ — `UserPromptSubmit` (keyword match, any standard,
   debounce-configurable) and `SessionStart` (must-read guarantee). Built, tested three
   tiers deep, merged.
3. ~~**Codex, safe path**~~ — no new code needed; already true via native `AGENTS.md` support.
4. ~~**Codex hook adapter**~~ — verification spike found the real contract matches Claude
   Code's exactly; shipped as a CLI tool-argument addition with zero new adapter logic.
5. **Extend the Shape A renderer** to Cursor, Copilot, Cline — each needs its own
   verification pass first (per the newly-elevated risk above), then a field-mapping entry.
6. **Shape B renderer** for Windsurf, Aider, Continue.
7. **Gemini CLI hook adapter** — validates the shared-core-plus-shim pattern generalizes past
   Claude Code/Codex; official/stable, and worth a verification spike the same way Codex got
   one before assuming its documented contract holds.
8. **opencode** — last. Needs a separate published JS/TS plugin package, not a config file a
   user drops in, which cuts against this project's single-binary premise. Not in the current
   priority list at all.
