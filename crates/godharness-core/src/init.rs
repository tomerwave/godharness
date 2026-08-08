use std::path::Path;

use crate::error::string_error;

const STARTER_CONFIG: &str = "version: 1\nsuites: [recommended@1]\n";

const STARTER_STANDARD: &str = "\
---
id: example
title: Example Standard
keywords: []
paths: []
must-read: false
supersedes: []
relates-to: []
---

## Rule

Replace this with a real rule your team has actually agreed on.

## Why

A starter standard exists so docs/godharness/ isn't empty; delete or replace it before it
misleads anyone into thinking it's a real standard.

## How to apply

Delete this file once you've written a real standard, or edit it into one.
";

string_error!(InitError, "", from: std::io::Error);

#[derive(Debug, PartialEq)]
pub struct InitReport {
    pub config_created: bool,
    pub starter_standard_created: bool,
}

fn write_if_absent(path: &Path, contents: &str) -> Result<bool, InitError> {
    if path.exists() {
        return Ok(false);
    }
    std::fs::write(path, contents)?;
    Ok(true)
}

pub fn run_init(root: &Path) -> Result<InitReport, InitError> {
    let config_created = write_if_absent(&root.join("godharness.yaml"), STARTER_CONFIG)?;

    let standards_dir = root.join("docs/godharness");
    std::fs::create_dir_all(&standards_dir)?;
    let starter_standard_created =
        write_if_absent(&standards_dir.join("example.md"), STARTER_STANDARD)?;

    Ok(InitReport {
        config_created,
        starter_standard_created,
    })
}
