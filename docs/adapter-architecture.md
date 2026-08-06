# Adapter architecture

Status: proposed — a design synthesized from research and discussion, not yet implemented.
Nothing in this document is a decision until code and tests back it; treat it the way
AGENTS.md says to treat anything else written down here.

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
| Claude Code | `.claude/rules/*.md` | Medium-low — one real bug confirmed | See [Claude Code](#claude-code). |
| GitHub Copilot | `.github/instructions/*.instructions.md`, `applyTo` frontmatter | Medium | Path-only, no "always" flag found. Also noted: path-scoped instructions are documented as supported for Copilot cloud agent / code review on GitHub.com — whether they apply the same way to local Copilot Chat/completions needs direct verification, not assumed from the docs. |
| Cline | `.clinerules/*.md` directory | Low — glob-scoping specifics not confirmed | Needs direct verification before relying on any scoping behavior. |
| Windsurf | `.windsurfrules`, freeform text | Medium | No scoping of any kind; whole-suite dump only. |
| Aider | `CONVENTIONS.md`, loaded via `--read`/`/read` | Medium | Whole-file, marked read-only for prompt caching. |
| Continue | Numbered `NN-name.md` files | Low | Ordering-only scoping found; no conditional logic. |

## Live-hook adapters

### Shared core, thin per-tool shim — but two structurally different shim kinds

**JSON-over-stdio tools** (Claude Code, Gemini CLI, and Codex if/when it stabilizes): the
tool spawns a command, writes JSON to its stdin, reads JSON from its stdout, and uses the
exit code as part of the decision protocol. The resolution logic is 100% shared — it's
`godharness context` — only field names and event names differ per tool. This reduces to one
shared core with a per-tool mapping table at the I/O boundary, structurally identical in
spirit to the static-file `FieldMapping` approach.

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

Hook surface is large (30+ events as of this research) — the relevant subset for godharness
is `UserPromptSubmit` (keyword-only standards, fires once per prompt, no matcher), and
`SessionStart` (must-read standards, guaranteed-timing injection). `PreToolUse`
(`Edit|Write` matcher) was viewstone's original approach for path-triggered standards, but:

- **`.claude/rules/*.md` (a native static mechanism, not present in viewstone's research) can
  replace `PreToolUse` for path-scoped standards entirely** — see the static-file table
  above. This is lower-risk than a live hook (no process spawn, no JSON contract to keep in
  sync with Claude Code's schema changes) and should be tried first.
- **Confirmed bug, not assumption**: the documented `.claude/rules/` frontmatter key is
  `paths:`, but it's silently broken — only the undocumented `globs:` key actually works
  ([anthropics/claude-code#17204](https://github.com/anthropics/claude-code/issues/17204)).
  Any renderer targeting this format must emit `globs:`, not `paths:`.
- **Unresolved from docs alone**: whether `PreToolUse`'s `hookSpecificOutput` genuinely
  supports `additionalContext` the way `UserPromptSubmit`/`PostToolUse`/`SessionStart` do —
  the general schema lists it as common across events, but the concrete `PreToolUse` example
  in the docs only shows `permissionDecision`/`updatedInput`, and the docs' own
  file-edit-context example uses `PostToolUse`, not `PreToolUse`. This needs a real
  invocation to settle, not another doc read (see [Testing strategy](#testing-strategy)). If
  `.claude/rules/` fully covers path-scoped standards, this question may not matter — but it
  should be answered before being relied on for anything else.

Distribution: Claude Code plugins can bundle `hooks/hooks.json` and be installed through the
marketplace mechanism, which is worth using once the hook adapter is proven, rather than
requiring users to hand-edit `.claude/settings.json`.

#### Codex

Materially riskier than Claude Code right now, not just a second instance of the same
pattern:

- Hooks are **experimental and opt-in** (`features.codex_hooks = true` in `~/.codex/config.toml`),
  shipped ~March 2026, not available on Windows, disabled by default.
- `PreToolUse` originally covered Bash only; `apply_patch` coverage was added later (~v0.123.0,
  April 2026) per an open feature request
  ([openai/codex#18491](https://github.com/openai/codex/issues/18491)) that also confirms
  `Edit`/`Write`/`Read`/web-fetch/MCP tool calls still don't trigger it as of that issue.
- A separate, officially-tracked open bug
  ([openai/codex#17532](https://github.com/openai/codex/issues/17532)): hooks configured via
  repo-local `.codex/config.toml` don't fire in interactive sessions at all.
- Third-party documentation of the exact `hooks.json` shape is internally contradictory
  (event names at file root with no wrapper, versus a `{"hooks": {...}}` wrapper) across
  sources that otherwise look credible. Nothing here should be trusted without a real
  `codex exec` run against it.

Codex reads `AGENTS.md` natively (confirmed — it's one of 28+ tools on the now-Linux-Foundation-
stewarded AGENTS.md convention, which this repository already follows). That gives Codex a
safe, zero-risk baseline today with no adapter code at all: keep this repository's own
AGENTS.md accurate (ongoing) and any repo installing godharness gets *some* Codex coverage
immediately. The hook adapter is real future work, but gated behind direct verification
first — see [Build order](#build-order).

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

**New, not something godlint has yet**: a Homebrew formula published to the existing
`tomerwave/homebrew-tap` repository, built from the same release binaries. That tap currently
holds one formula (`convert-to-md`, unrelated, GoReleaser-generated) — godharness's would be
hand-written in the same style (`on_macos`/`on_linux` blocks, per-arch `url`/`sha256`), not
GoReleaser output, since this is a Rust workspace built directly in CI rather than through
GoReleaser's Go-oriented pipeline.

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

- Claude Code: whether `PreToolUse` supports `additionalContext` (see
  [Claude Code](#claude-code)).
- Codex: whether the real `hooks.json` shape matches either contradictory third-party
  description, and whether the repo-local-config bug
  ([openai/codex#17532](https://github.com/openai/codex/issues/17532)) affects the intended
  installation path.
- Cursor/Copilot/Cline: frontmatter field behavior sourced from third-party docs only, not
  independently confirmed the way the Claude Code `.claude/rules/` bug was.
- Every "confidence: medium/low" row above is real future verification work, not a settled
  fact this document is asserting.

## Build order

Ranked by risk and dependency, with Claude Code and Codex prioritized per direct instruction:

1. **Generic static-file renderer** (Shape A) — foundational, lowest risk (no live process, no
   experimental flags). First output: Claude Code's `.claude/rules/*.md`.
2. **Claude Code hook adapter** — `UserPromptSubmit` (keyword-only) and `SessionStart`
   (must-read timing). Closes the two gaps the static renderer can't cover. Claude Code fully
   covered after this step.
3. **Codex, safe path** — no new adapter code, just keep this repository's (and any installing
   repo's) AGENTS.md accurate, since Codex already reads it natively.
4. **Codex hook adapter** — gated behind a real verification spike (an actual `codex exec` run
   with `features.codex_hooks = true`) before writing adapter code, given the contradictory
   third-party docs and the two open upstream issues.
5. **Extend the Shape A renderer** to Cursor, Copilot, Cline — near-free once step 1 exists,
   each is a new field-mapping entry.
6. **Shape B renderer** for Windsurf, Aider, Continue.
7. **Gemini CLI hook adapter** — validates the shared-core-plus-shim pattern generalizes past
   Claude Code; official/stable, unlike Codex's current state.
8. **opencode** — last. Needs a separate published JS/TS plugin package, not a config file a
   user drops in, which cuts against this project's single-binary premise. Not in the current
   priority list at all.
