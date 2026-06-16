mod storage;
mod wine;

use anyhow::Result;
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use std::path::PathBuf;
use storage::{load_wines, save_wines};
use wine::{Wine, WineInput};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about, long_about = None)]
struct Args {
    #[arg(short, long)]
    data: Option<String>,
    #[clap(subcommand)]
    subcommand: Subcmd,
}

fn get_default_data_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "whyno", "whyno") {
        proj_dirs.data_dir().join("wines.json")
    } else {
        PathBuf::from("./whyno.json")
    }
}

#[derive(Subcommand, Debug)]
enum Subcmd {
    Add(AddArgs),
    List(ListArgs),
    Show { id: u64 },
    Update(UpdateArgs),
    Remove { id: u64 },
    Stats,
}

#[derive(Parser, Debug)]
struct ListArgs {
    #[arg(long)]
    tag: Option<String>,
    #[arg(long)]
    grape: Option<String>,
    #[arg(long)]
    country: Option<String>,
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=5))]
    min_rating: Option<u8>,
    #[arg(long)]
    query: Option<String>,
}

#[derive(Parser, Debug)]
struct AddArgs {
    name: String,
    #[arg(short, long)]
    vintage: Option<u32>,
    #[arg(short, long)]
    producer: Option<String>,
    #[arg(long)]
    region: Option<String>,
    #[arg(short, long)]
    country: Option<String>,
    #[arg(short, long)]
    grape: Option<String>,
    #[arg(short, long)]
    rating: Option<u8>,
    #[arg(short, long)]
    notes: Option<String>,
    #[arg(short, long, value_delimiter = ',')]
    tag: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
struct UpdateArgs {
    id: u64,
    #[arg(short, long)]
    vintage: Option<u32>,
    #[arg(short, long)]
    producer: Option<String>,
    #[arg(long)]
    region: Option<String>,
    #[arg(short, long)]
    country: Option<String>,
    #[arg(short, long)]
    grape: Option<String>,
    #[arg(short, long)]
    rating: Option<u8>,
    #[arg(short, long)]
    notes: Option<String>,
    #[arg(short, long, value_delimiter = ',')]
    tag: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let data_path = args
        .data
        .map(PathBuf::from)
        .unwrap_or_else(get_default_data_path);

    match args.subcommand {
        Subcmd::Add(add_args) => {
            let mut wines = load_wines(&data_path)?;
            let next_id = Wine::next_id(&wines);

            let input = WineInput {
                name: add_args.name.clone(),
                producer: add_args.producer,
                vintage: add_args.vintage,
                price: None,
                purchase_date: None,
                drink_by: None,
                region: add_args.region,
                country: add_args.country,
                grape: add_args.grape,
                rating: add_args.rating,
                notes: add_args.notes,
                tags: add_args.tag,
            };

            match Wine::from_input(next_id, input) {
                Ok(wine) => {
                    wines.push(wine.clone());
                    save_wines(&data_path, &wines)?;
                    println!("Added wine: {} (ID: {})", wine.name, wine.id);
                    if let Some(rating) = wine.rating {
                        println!("  Rating: {}", rating);
                    }
                    if let Some(notes) = &wine.notes {
                        println!("  Notes: {}", notes);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Subcmd::List(list_args) => {
            let wines = load_wines(&data_path)?;
            let filtered: Vec<_> = wines
                .iter()
                .filter(|wine| {
                    if let Some(tag) = &list_args.tag {
                        wine.tags.as_ref().is_some_and(|tags| {
                            tags.iter().any(|t| t.to_lowercase() == tag.to_lowercase())
                        })
                    } else {
                        true
                    }
                })
                .filter(|wine| {
                    if let Some(grape) = &list_args.grape {
                        wine.grapes.as_ref().is_some_and(|grapes| {
                            grapes
                                .iter()
                                .any(|g| g.to_lowercase() == grape.to_lowercase())
                        })
                    } else {
                        true
                    }
                })
                .filter(|wine| {
                    if let Some(country) = &list_args.country {
                        wine.country
                            .as_ref()
                            .is_some_and(|c| c.to_lowercase() == country.to_lowercase())
                    } else {
                        true
                    }
                })
                .filter(|wine| {
                    if let Some(min_rating) = list_args.min_rating {
                        wine.rating.is_some_and(|r| r >= min_rating)
                    } else {
                        true
                    }
                })
                .filter(|wine| {
                    if let Some(query) = &list_args.query {
                        let query_lower = query.to_lowercase();
                        wine.name.to_lowercase().contains(&query_lower)
                            || wine
                                .producer
                                .as_ref()
                                .is_some_and(|p| p.to_lowercase().contains(&query_lower))
                            || wine
                                .region
                                .as_ref()
                                .is_some_and(|r| r.to_lowercase().contains(&query_lower))
                            || wine
                                .country
                                .as_ref()
                                .is_some_and(|c| c.to_lowercase().contains(&query_lower))
                            || wine.grapes.as_ref().is_some_and(|g| {
                                g.iter().any(|gr| gr.to_lowercase().contains(&query_lower))
                            })
                            || wine
                                .notes
                                .as_ref()
                                .is_some_and(|n| n.to_lowercase().contains(&query_lower))
                            || wine.tags.as_ref().is_some_and(|t| {
                                t.iter()
                                    .any(|tag| tag.to_lowercase().contains(&query_lower))
                            })
                    } else {
                        true
                    }
                })
                .collect();

            if filtered.is_empty() {
                println!("No wines found matching criteria. Add one with `whyno add <name>`");
            } else {
                println!("Found {} wine(s):\n", filtered.len());
                for wine in filtered {
                    match wine.rating {
                        Some(rating) => {
                            println!("{}. {} — {}/5", wine.id, wine.name, rating);
                        }
                        None => {
                            println!("{}. {} — Unrated", wine.id, wine.name);
                        }
                    }
                    if let Some(producer) = &wine.producer {
                        println!("   Producer: {}", producer);
                    }
                    if let Some(vintage) = wine.vintage {
                        println!("   Vintage: {}", vintage);
                    }
                    if let Some(region) = &wine.region {
                        let location = if let Some(country) = &wine.country {
                            format!("{}, {}", region, country)
                        } else {
                            region.clone()
                        };
                        println!("   Region: {}", location);
                    } else if let Some(country) = &wine.country {
                        println!("   Country: {}", country);
                    }
                    if let Some(grapes) = &wine.grapes {
                        println!("   Grapes: {}", grapes.join(", "));
                    }
                    if let Some(notes) = &wine.notes {
                        println!("   Notes: {}", notes);
                    }
                    println!();
                }
            }
        }
        Subcmd::Show { id } => {
            let wines = load_wines(&data_path)?;
            if let Some(wine) = wines.iter().find(|w| w.id == id) {
                match wine.rating {
                    Some(rating) => {
                        println!("{} — {}/5", wine.name, rating);
                    }
                    None => {
                        println!("{} — Unrated", wine.name);
                    }
                }
                if let Some(producer) = &wine.producer {
                    println!("Producer: {}", producer);
                }
                if let Some(vintage) = wine.vintage {
                    println!("Vintage: {}", vintage);
                }
                if let Some(region) = &wine.region {
                    let location = if let Some(country) = &wine.country {
                        format!("{}, {}", region, country)
                    } else {
                        region.clone()
                    };
                    println!("Region: {}", location);
                } else if let Some(country) = &wine.country {
                    println!("Country: {}", country);
                }
                if let Some(grapes) = &wine.grapes {
                    println!("Grapes: {}", grapes.join(", "));
                }
                if let Some(notes) = &wine.notes {
                    println!("Notes: {}", notes);
                }
                if let Some(tags) = &wine.tags {
                    println!("Tags: {}", tags.join(", "));
                }
            } else {
                eprintln!("Error: Wine with ID {} not found", id);
                std::process::exit(1);
            }
        }
        Subcmd::Update(update_args) => {
            let mut wines = load_wines(&data_path)?;
            if let Some(pos) = wines.iter().position(|w| w.id == update_args.id) {
                let wine = &mut wines[pos];

                // Update fields if provided
                if let Some(vintage) = update_args.vintage {
                    if !(1900..=2100).contains(&vintage) {
                        eprintln!("Error: Vintage must be between 1900 and 2100");
                        std::process::exit(1);
                    }
                    wine.vintage = Some(vintage);
                }
                if let Some(producer) = update_args.producer {
                    wine.producer = Some(producer.trim().to_string());
                }
                if let Some(region) = update_args.region {
                    wine.region = Some(region.trim().to_string());
                }
                if let Some(country) = update_args.country {
                    wine.country = Some(country.trim().to_string());
                }
                if let Some(grape) = update_args.grape {
                    wine.grapes = Some(vec![grape.trim().to_string()]);
                }
                if let Some(rating) = update_args.rating {
                    if !(1..=5).contains(&rating) {
                        eprintln!("Error: Rating must be between 1 and 5");
                        std::process::exit(1);
                    }
                    wine.rating = Some(rating);
                }
                if let Some(notes) = update_args.notes {
                    wine.notes = Some(notes.trim().to_string());
                }
                if let Some(tags) = update_args.tag {
                    wine.tags = Some(tags.iter().map(|t| t.trim().to_string()).collect());
                }

                let wine_name = wine.name.clone();
                save_wines(&data_path, &wines)?;
                println!("Updated wine: {}", wine_name);
            } else {
                eprintln!("Error: Wine with ID {} not found", update_args.id);
                std::process::exit(1);
            }
        }
        Subcmd::Remove { id } => {
            let mut wines = load_wines(&data_path)?;
            if let Some(pos) = wines.iter().position(|w| w.id == id) {
                let removed = wines.remove(pos);
                save_wines(&data_path, &wines)?;
                println!("Removed wine: {}", removed.name);
            } else {
                eprintln!("Error: Wine with ID {} not found", id);
                std::process::exit(1);
            }
        }
        Subcmd::Stats => {
            let wines = load_wines(&data_path)?;
            let count = wines.len();

            if count > 0 {
                let sum: f64 = wines
                    .iter()
                    .filter_map(|w| w.rating)
                    .map(|r| r as f64)
                    .sum();
                let rated_count = wines.iter().filter(|w| w.rating.is_some()).count();
                println!("Total wines saved: {}", count);
                if rated_count > 0 {
                    println!("Average rating: {:.2}", sum / rated_count as f64);
                } else {
                    println!("Average rating: N/A (no rated wines)");
                }
            } else {
                println!("No wines saved yet.");
            }
        }
    }

    Ok(())
}
