use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wine {
    pub id: u64,
    pub name: String,
    pub producer: Option<String>,
    pub vintage: Option<u32>,
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
    pub region: Option<String>,
    pub country: Option<String>,
    pub grape: Option<String>,
    pub rating: Option<u8>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Wine {
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

        Ok(Wine {
            id,
            name: input.name.trim().to_string(),
            producer: input.producer.map(|s| s.trim().to_string()),
            vintage: input.vintage,
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
