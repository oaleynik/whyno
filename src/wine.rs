use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wine {
    pub id: u64,
    pub name: String,
    pub producer: Option<String>,
    pub vintage: Option<u32>,
    #[serde(default)]
    pub price: Option<f32>,
    #[serde(default)]
    pub purchase_date: Option<String>,
    #[serde(default)]
    pub drink_by: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub grapes: Option<Vec<String>>,
    pub rating: Option<u8>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct WineInput {
    pub name: String,
    pub producer: Option<String>,
    pub vintage: Option<u32>,
    pub price: Option<f32>,
    pub purchase_date: Option<String>,
    pub drink_by: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub grape: Option<String>,
    pub rating: Option<u8>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Wine {
    pub fn next_id(wines: &[Wine]) -> u64 {
        wines.iter().map(|w| w.id).max().unwrap_or(0) + 1
    }

    pub fn from_input(id: u64, input: WineInput) -> Result<Self> {
        // Validate name
        if input.name.trim().is_empty() {
            bail!("Wine name cannot be empty");
        }

        // Validate rating
        if let Some(rating) = input.rating
            && !(1..=5).contains(&rating)
        {
            bail!("Rating must be between 1 and 5");
        }

        // Validate vintage (if provided, reasonable range)
        if let Some(vintage) = input.vintage
            && !(1900..=2100).contains(&vintage)
        {
            bail!("Vintage must be between 1900 and 2100");
        }

        // Validate price
        if let Some(price) = input.price {
            if !price.is_finite() {
                bail!("Price must be finite");
            }
            if price < 0.0 {
                bail!("Price must be non-negative");
            }
        }

        let purchase_date = normalize_optional_date(input.purchase_date, "Purchase date")?;
        let drink_by = normalize_optional_date(input.drink_by, "Drink-by date")?;

        Ok(Wine {
            id,
            name: input.name.trim().to_string(),
            producer: input.producer.map(|s| s.trim().to_string()),
            vintage: input.vintage,
            price: input.price,
            purchase_date,
            drink_by,
            region: input.region.map(|s| s.trim().to_string()),
            country: input.country.map(|s| s.trim().to_string()),
            grapes: input.grape.map(|g| vec![g.trim().to_string()]),
            rating: input.rating,
            notes: input.notes.map(|s| s.trim().to_string()),
            tags: input
                .tags
                .map(|tags| tags.iter().map(|t| t.trim().to_string()).collect()),
        })
    }
}

fn normalize_optional_date(date: Option<String>, field_name: &str) -> Result<Option<String>> {
    date.map(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            bail!("{} cannot be empty", field_name);
        }
        Ok(trimmed.to_string())
    })
    .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_name_validation() {
        let input = WineInput {
            name: "   ".to_string(),
            producer: None,
            vintage: None,
            price: None,
            purchase_date: None,
            drink_by: None,
            region: None,
            country: None,
            grape: None,
            rating: None,
            notes: None,
            tags: None,
        };

        let result = Wine::from_input(1, input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Wine name cannot be empty");
    }

    #[test]
    fn test_invalid_rating_validation() {
        let input = WineInput {
            name: "Test Wine".to_string(),
            producer: None,
            vintage: None,
            price: None,
            purchase_date: None,
            drink_by: None,
            region: None,
            country: None,
            grape: None,
            rating: Some(6),
            notes: None,
            tags: None,
        };

        let result = Wine::from_input(1, input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Rating must be between 1 and 5"
        );
    }

    #[test]
    fn test_invalid_vintage_validation() {
        let input = WineInput {
            name: "Test Wine".to_string(),
            producer: None,
            vintage: Some(1899),
            price: None,
            purchase_date: None,
            drink_by: None,
            region: None,
            country: None,
            grape: None,
            rating: None,
            notes: None,
            tags: None,
        };

        let result = Wine::from_input(1, input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Vintage must be between 1900 and 2100"
        );
    }

    #[test]
    fn test_valid_wine_creation() {
        let input = WineInput {
            name: "  Test Wine  ".to_string(),
            producer: Some("  Test Producer  ".to_string()),
            vintage: Some(2020),
            price: None,
            purchase_date: None,
            drink_by: None,
            region: Some("  Napa Valley  ".to_string()),
            country: Some("  USA  ".to_string()),
            grape: Some("  Cabernet Sauvignon  ".to_string()),
            rating: Some(4),
            notes: Some("  Great wine  ".to_string()),
            tags: Some(vec!["  red  ".to_string(), "  bold  ".to_string()]),
        };

        let wine = Wine::from_input(42, input).unwrap();
        assert_eq!(wine.id, 42);
        assert_eq!(wine.name, "Test Wine");
        assert_eq!(wine.producer, Some("Test Producer".to_string()));
        assert_eq!(wine.vintage, Some(2020));
        assert_eq!(wine.region, Some("Napa Valley".to_string()));
        assert_eq!(wine.country, Some("USA".to_string()));
        assert_eq!(wine.grapes, Some(vec!["Cabernet Sauvignon".to_string()]));
        assert_eq!(wine.rating, Some(4));
        assert_eq!(wine.notes, Some("Great wine".to_string()));
        assert_eq!(wine.tags, Some(vec!["red".to_string(), "bold".to_string()]));
    }

    #[test]
    fn test_valid_wine_creation_with_richer_fields() {
        let input = WineInput {
            name: "Test Wine".to_string(),
            producer: None,
            vintage: None,
            price: Some(42.50),
            purchase_date: Some(" 2024-06-01 ".to_string()),
            drink_by: Some(" 2030-01-01 ".to_string()),
            region: None,
            country: None,
            grape: None,
            rating: None,
            notes: None,
            tags: None,
        };

        let wine = Wine::from_input(42, input).unwrap();
        assert_eq!(wine.price, Some(42.50));
        assert_eq!(wine.purchase_date, Some("2024-06-01".to_string()));
        assert_eq!(wine.drink_by, Some("2030-01-01".to_string()));
    }

    #[test]
    fn test_negative_price_validation() {
        let input = WineInput {
            name: "Test Wine".to_string(),
            producer: None,
            vintage: None,
            price: Some(-1.0),
            purchase_date: None,
            drink_by: None,
            region: None,
            country: None,
            grape: None,
            rating: None,
            notes: None,
            tags: None,
        };

        let result = Wine::from_input(1, input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Price must be non-negative"
        );
    }

    #[test]
    fn test_non_finite_price_validation() {
        let input = WineInput {
            name: "Test Wine".to_string(),
            producer: None,
            vintage: None,
            price: Some(f32::NAN),
            purchase_date: None,
            drink_by: None,
            region: None,
            country: None,
            grape: None,
            rating: None,
            notes: None,
            tags: None,
        };

        let result = Wine::from_input(1, input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Price must be finite");
    }

    #[test]
    fn test_empty_purchase_date_validation() {
        let input = WineInput {
            name: "Test Wine".to_string(),
            producer: None,
            vintage: None,
            price: None,
            purchase_date: Some("   ".to_string()),
            drink_by: None,
            region: None,
            country: None,
            grape: None,
            rating: None,
            notes: None,
            tags: None,
        };

        let result = Wine::from_input(1, input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Purchase date cannot be empty"
        );
    }

    #[test]
    fn test_empty_drink_by_validation() {
        let input = WineInput {
            name: "Test Wine".to_string(),
            producer: None,
            vintage: None,
            price: None,
            purchase_date: None,
            drink_by: Some("   ".to_string()),
            region: None,
            country: None,
            grape: None,
            rating: None,
            notes: None,
            tags: None,
        };

        let result = Wine::from_input(1, input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Drink-by date cannot be empty"
        );
    }

    #[test]
    fn test_deserialize_wine_without_richer_fields() {
        let wine: Wine = serde_json::from_str(
            r#"{
                "id": 1,
                "name": "Cellar Classic",
                "producer": null,
                "vintage": null,
                "region": null,
                "country": null,
                "grapes": null,
                "rating": null,
                "notes": null,
                "tags": null
            }"#,
        )
        .unwrap();

        assert_eq!(wine.price, None);
        assert_eq!(wine.purchase_date, None);
        assert_eq!(wine.drink_by, None);
    }

    #[test]
    fn test_next_id_empty() {
        assert_eq!(Wine::next_id(&[]), 1);
    }

    #[test]
    fn test_next_id_single() {
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
        assert_eq!(Wine::next_id(&wines), 2);
    }

    #[test]
    fn test_next_id_multiple() {
        let wines = vec![
            Wine {
                id: 1,
                name: "Test 1".to_string(),
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
            Wine {
                id: 3,
                name: "Test 3".to_string(),
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
            Wine {
                id: 2,
                name: "Test 2".to_string(),
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
        assert_eq!(Wine::next_id(&wines), 4);
    }
}
