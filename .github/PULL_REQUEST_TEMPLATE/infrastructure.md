## Summary

<!-- What this changes in the build, CI, tooling, or documentation, and why. -->

## Effect on contributors

<!--
Does this change what a contributor has to run locally, or what CI will reject? If so,
say so plainly and confirm the documentation matches.
-->

## Validation

<!--
For a workflow change, state how you know it works. A workflow that has never run is not
evidence. For a documentation change, confirm the claims against the code.
-->

## Checklist

- [ ] No suite schema, `recommended@1` content, or resolver behavior changed. (If it did,
      this is the wrong template.)
- [ ] Every workflow declares `permissions` and pins one Rust toolchain.
- [ ] Local commands in `docs/local-development.md` still match what CI runs.
