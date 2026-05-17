use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about, long_about = None)]
struct Args {
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

    match args.subcommand {
        Subcmd::Add(add_args) => {
            println!("Adding wine: {}", add_args.name);
            if let Some(rating) = add_args.rating {
                println!("  Rating: {}", rating);
            }
            if let Some(notes) = add_args.notes {
                println!("  Notes: {}", notes);
            }
            println!("(Storage not yet implemented)");
        }
        Subcmd::List => println!("Listing wines (storage not yet implemented)"),
    }

    Ok(())
}
