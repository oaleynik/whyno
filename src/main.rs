mod storage;
mod wine;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use storage::{load_wines, save_wines};
use wine::{Wine, WineInput};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./whyno.json")]
    data: String,
    #[clap(subcommand)]
    subcommand: Subcmd,
}

#[derive(Subcommand, Debug)]
enum Subcmd {
    Add(AddArgs),
    List(ListArgs),
    Show { id: u64 },
    Remove { id: u64 },
}

#[derive(Parser, Debug)]
struct ListArgs {
    #[arg(long)]
    tag: Option<String>,
    #[arg(long)]
    grape: Option<String>,
    #[arg(long)]
    country: Option<String>,
    #[arg(long)]
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

fn main() -> Result<()> {
    let args = Args::parse();
    let data_path = PathBuf::from(&args.data);

    match args.subcommand {
        Subcmd::Add(add_args) => {
            println!("Using data file: {}", args.data);
            let mut wines = load_wines(&data_path)?;
            let next_id = wines.iter().map(|w| w.id).max().unwrap_or(0) + 1;

            let input = WineInput {
                name: add_args.name.clone(),
                producer: add_args.producer,
                vintage: add_args.vintage,
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
                        wine.tags.as_ref().map_or(false, |tags| {
                            tags.iter().any(|t| t.to_lowercase() == tag.to_lowercase())
                        })
                    } else {
                        true
                    }
                })
                .filter(|wine| {
                    if let Some(grape) = &list_args.grape {
                        wine.grapes.as_ref().map_or(false, |grapes| {
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
                            .map_or(false, |c| c.to_lowercase() == country.to_lowercase())
                    } else {
                        true
                    }
                })
                .filter(|wine| {
                    if let Some(min_rating) = list_args.min_rating {
                        wine.rating.map_or(false, |r| r >= min_rating)
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
                                .map_or(false, |p| p.to_lowercase().contains(&query_lower))
                            || wine
                                .region
                                .as_ref()
                                .map_or(false, |r| r.to_lowercase().contains(&query_lower))
                            || wine
                                .country
                                .as_ref()
                                .map_or(false, |c| c.to_lowercase().contains(&query_lower))
                            || wine.grapes.as_ref().map_or(false, |g| {
                                g.iter().any(|gr| gr.to_lowercase().contains(&query_lower))
                            })
                            || wine
                                .notes
                                .as_ref()
                                .map_or(false, |n| n.to_lowercase().contains(&query_lower))
                            || wine.tags.as_ref().map_or(false, |t| {
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
                    println!(
                        "{}. {} — {}/5",
                        wine.id,
                        wine.name,
                        wine.rating.unwrap_or(0)
                    );
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
                println!("{} — {}/5", wine.name, wine.rating.unwrap_or(0));
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
    }

    Ok(())
}
