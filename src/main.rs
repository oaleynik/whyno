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
    List,
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
        Subcmd::List => {
            println!("Using data file: {}", args.data);
            let wines = load_wines(&data_path)?;
            println!(
                "Loaded {} wines (storage works but list command not yet implemented)",
                wines.len()
            );
        }
    }

    Ok(())
}
