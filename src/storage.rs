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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_missing_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().join("nonexistent.json");
        assert!(!path.exists());

        let result = load_wines(&path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_load_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), "").unwrap();

        let result = load_wines(temp_file.path());
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_save_load_roundtrip() {
        let temp_file = NamedTempFile::new().unwrap();
        let wines = vec![
            Wine {
                id: 1,
                name: "Test Wine".to_string(),
                producer: Some("Test Producer".to_string()),
                vintage: Some(2020),
                price: None,
                purchase_date: None,
                drink_by: None,
                region: Some("Napa Valley".to_string()),
                country: Some("USA".to_string()),
                grapes: Some(vec!["Cabernet Sauvignon".to_string()]),
                rating: Some(4),
                notes: Some("Great wine".to_string()),
                tags: Some(vec!["red".to_string(), "bold".to_string()]),
            },
            Wine {
                id: 2,
                name: "Another Wine".to_string(),
                producer: None,
                vintage: None,
                price: None,
                purchase_date: None,
                drink_by: None,
                region: None,
                country: None,
                grapes: None,
                rating: None,
                notes: None,
                tags: None,
            },
        ];

        // Save
        save_wines(temp_file.path(), &wines).unwrap();

        // Load
        let loaded = load_wines(temp_file.path()).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, 1);
        assert_eq!(loaded[0].name, "Test Wine");
        assert_eq!(loaded[1].id, 2);
        assert_eq!(loaded[1].name, "Another Wine");
    }

    #[test]
    fn test_load_invalid_json() {
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), "{ invalid json }").unwrap();

        let result = load_wines(temp_file.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to parse"));
    }

    #[test]
    fn test_save_creates_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nested_path = temp_dir
            .path()
            .join("nested")
            .join("dir")
            .join("wines.json");
        assert!(!nested_path.parent().unwrap().exists());

        let wines = vec![Wine {
            id: 1,
            name: "Test".to_string(),
            producer: None,
            vintage: None,
            price: None,
            purchase_date: None,
            drink_by: None,
            region: None,
            country: None,
            grapes: None,
            rating: None,
            notes: None,
            tags: None,
        }];

        save_wines(&nested_path, &wines).unwrap();
        assert!(nested_path.exists());
    }
}
