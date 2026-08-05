# Local development

Godharness is a Rust workspace: `godharness-core` holds the config model and resolver,
`godharness-cli` builds the `godharness` binary. It requires Rust `1.97.1`, which
`rust-toolchain.toml` pins, so [rustup](https://rustup.rs/) installs the right one on first
build.

## The checks CI runs

Run these before opening a pull request. CI runs the same ones, and a failure here is a
failure there:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

CI also runs `godlint` against this repository through the published GitHub Action
(`tomerwave/godlint@v1`), rather than a locally built binary — there's nothing to run for
that check locally beyond installing `godlint` yourself if you want to check ahead of CI.

## Running the binary you just built

```bash
cargo run -p godharness-cli --bin godharness -- --version
cargo run -p godharness-cli --bin godharness -- check
cargo run -p godharness-cli --bin godharness -- doctor
cargo run -p godharness-cli --bin godharness -- context --prompt "add a new endpoint"
```

`check`, `doctor`, and `init` are human- and CI-facing; running them by hand is the normal
way to use them. `context` is different: it's the JSON contract agent adapters call on every
prompt or file edit, not something a person types day to day. All four are stubs today —
none of them do real work yet, since the resolver and standard schema are open decisions
(see the repository README).

## Tests

Crate contracts live in `crates/<crate>/tests/`; no test code belongs in `src/`.
`godharness-core`'s unit tests live in `src/lib.rs` under `#[cfg(test)]` for now, since the
crate is small enough that splitting them out would just add indirection — revisit once the
crate grows past a stub.

## Two conventions worth knowing

**`context`'s JSON output is a stable contract.** Every adapter depends on its shape
(currently: a JSON array of strings on stdout). Changing that shape is a breaking change and
needs to be called out as one, even while the array is always empty.

**`clippy.toml`'s `allow-unwrap-in-tests`/`allow-expect-in-tests`.** The workspace denies
`unwrap`/`expect` in the `clippy` lint group at the `warn` level, which `-D warnings`
promotes to an error. Those two `clippy.toml` settings exempt test code, so tests can use
`.unwrap()`/`.expect(...)` normally while production code in `src/` cannot.
