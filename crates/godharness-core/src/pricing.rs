use std::collections::HashMap;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::stats::UsageEvent;

const CANONICAL_PROVIDER_ORDER: &[&str] = &["anthropic", "openai"];

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ModelRate {
    pub input: f64,
    pub output: f64,
    pub cache_read: f64,
    pub cache_write: f64,
}

#[derive(Debug, Deserialize)]
struct Snapshot {
    source: String,
    fetched_at: String,
    providers: HashMap<String, HashMap<String, ModelRate>>,
}

static SNAPSHOT: LazyLock<Snapshot> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../pricing/snapshot.json"))
        .unwrap_or_else(|error| panic!("pricing: embedded snapshot failed to parse: {error}"))
});

pub fn snapshot_source() -> &'static str {
    &SNAPSHOT.source
}

pub fn snapshot_fetched_at() -> &'static str {
    &SNAPSHOT.fetched_at
}

pub fn rate_for(model_id: &str) -> Option<ModelRate> {
    CANONICAL_PROVIDER_ORDER
        .iter()
        .find_map(|provider| SNAPSHOT.providers.get(*provider)?.get(model_id).copied())
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModelCostEntry {
    pub model: String,
    pub tokens: u64,
    pub estimated_usd: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CostReport {
    pub priced: Vec<ModelCostEntry>,
    pub unpriced_tokens: u64,
}

fn tokens_by_model<'a>(
    events: &'a [UsageEvent],
    override_model: Option<&'a str>,
) -> HashMap<&'a str, u64> {
    let mut totals: HashMap<&str, u64> = HashMap::new();
    for event in events {
        let Some(model) = override_model.or(event.model.as_deref()) else {
            continue;
        };
        *totals.entry(model).or_insert(0) += u64::from(event.approx_tokens);
    }
    totals
}

fn priced_entry(model: &str, tokens: u64) -> Option<ModelCostEntry> {
    let rate = rate_for(model)?;
    let estimated_usd = (tokens as f64) / 1_000_000.0 * rate.input;
    Some(ModelCostEntry {
        model: model.to_string(),
        tokens,
        estimated_usd,
    })
}

pub fn estimate_cost(events: &[UsageEvent], override_model: Option<&str>) -> CostReport {
    let unmodeled_tokens: u64 = if override_model.is_some() {
        0
    } else {
        events
            .iter()
            .filter(|event| event.model.is_none())
            .map(|event| u64::from(event.approx_tokens))
            .sum()
    };

    let mut priced = Vec::new();
    let mut unpriced_tokens = unmodeled_tokens;
    for (model, tokens) in tokens_by_model(events, override_model) {
        match priced_entry(model, tokens) {
            Some(entry) => priced.push(entry),
            None => unpriced_tokens += tokens,
        }
    }
    priced.sort_by(|a, b| a.model.cmp(&b.model));

    CostReport {
        priced,
        unpriced_tokens,
    }
}
