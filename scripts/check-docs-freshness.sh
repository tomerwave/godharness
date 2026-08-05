#!/usr/bin/env bash
set -euo pipefail

stale_claims=(
  "none of them doing real work yet"
  "a stub CLI"
  "even while it's always an empty array today"
  "Exact standard frontmatter schema and selectors."
  "Priority, precedence, and conflict-reporting rules."
  "How a project extends or overrides a recommended standard."
  "The minimal universal core that every task receives."
)

checked_docs=(README.md AGENTS.md)

exit_code=0
for doc in "${checked_docs[@]}"; do
  for claim in "${stale_claims[@]}"; do
    if grep -qF "$claim" "$doc"; then
      echo "::error file=$doc::stale claim reintroduced: $claim"
      exit_code=1
    fi
  done
done

exit "$exit_code"
