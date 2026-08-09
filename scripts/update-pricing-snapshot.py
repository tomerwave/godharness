#!/usr/bin/env python3
"""Regenerate the bundled model-pricing snapshot from models.dev.

Restricted to first-party providers matching godharness's supported adapters
(Anthropic for Claude Code, OpenAI for Codex) - models.dev also lists resold
and proxied entries for the same model id under other providers at different
prices, which would make a single flattened lookup silently wrong.

Usage: python3 scripts/update-pricing-snapshot.py
"""

from __future__ import annotations

import datetime
import json
import urllib.request
from pathlib import Path

SOURCE_URL = "https://models.dev/api.json"
SOURCE_NOTE = (
    "https://models.dev/api.json (MIT licensed, github.com/anomalyco/models.dev)"
)
CANONICAL_PROVIDERS = ["anthropic", "openai"]
OUTPUT = Path(__file__).resolve().parent.parent / "crates" / "godharness-core" / "pricing" / "snapshot.json"


def fetch_raw() -> dict:
    request = urllib.request.Request(SOURCE_URL, headers={"User-Agent": "godharness-pricing-snapshot"})
    with urllib.request.urlopen(request) as response:
        return json.load(response)


def trim(raw: dict) -> dict:
    providers = {}
    for provider_id in CANONICAL_PROVIDERS:
        models = {}
        for model_id, model in raw.get(provider_id, {}).get("models", {}).items():
            cost = model.get("cost")
            if not cost:
                continue
            models[model_id] = {
                "input": cost.get("input", 0),
                "output": cost.get("output", 0),
                "cache_read": cost.get("cache_read", 0),
                "cache_write": cost.get("cache_write", 0),
            }
        providers[provider_id] = models
    return providers


def main() -> None:
    raw = fetch_raw()
    providers = trim(raw)

    snapshot = {
        "source": SOURCE_NOTE,
        "fetched_at": datetime.date.today().isoformat(),
        "note": (
            "Restricted to first-party providers (anthropic, openai) matching "
            "godharness's supported adapters, to avoid reseller/proxy price "
            "ambiguity present in models.dev's other provider entries."
        ),
        "providers": providers,
    }

    OUTPUT.write_text(json.dumps(snapshot, indent=2, sort_keys=True) + "\n")

    total = sum(len(models) for models in providers.values())
    print(f"wrote {OUTPUT} ({total} models across {len(providers)} providers)")


if __name__ == "__main__":
    main()
