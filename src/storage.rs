use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::wine::Wine;

pub fn load_wines(path: &Path) -> Result<Vec<Wine>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read wine data from {}", path.display()))?;

    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse wine data from {}", path.display()))
}

pub fn save_wines(path: &Path, wines: &[Wine]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory for {}", path.display()))?;
    }

    let json = serde_json::to_string_pretty(wines).context("Failed to serialize wine data")?;

    fs::write(path, json)
        .with_context(|| format!("Failed to write wine data to {}", path.display()))
}
