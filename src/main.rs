use clap::{Parser, Subcommand};
use colored::Colorize;
use sem::{
    expense::Expense,
    stats::{generate_stats, recent_transactions},
    utils::{init_source_file, source_file_path},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "sem")]
#[command(about = "A simple expense tracker that holds your data locally", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add new expense
    NEW,
    /// Display stats
    STATS,
}

fn main() {
    // Simple Expense manager
    println!(
        "{}",
        "
            ███████╗███████╗███╗   ███╗
            ██╔════╝██╔════╝████╗ ████║
            ███████╗█████╗  ██╔████╔██║
            ╚════██║██╔══╝  ██║╚██╔╝██║
            ███████║███████╗██║ ╚═╝ ██║
            ╚══════╝╚══════╝╚═╝     ╚═╝"
            .bright_magenta()
    );

    init_source_file(&source_file_path());
    println!();
    println!("Recent transactions");
    recent_transactions();
    println!();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::NEW) => {
            match Expense::new() {
                Ok(expense) => {
                    println!("✅ Added {:#?}", expense);
                }
                Err(e) => {
                    // Handle the error case
                    println!("⚠️ {}", e);
                }
            }
        }
        Some(Commands::STATS) => generate_stats(),
        None => {
            println!("{} {} \n", "⚠️", "Run sem -h for usage information");
        }
    }
}
