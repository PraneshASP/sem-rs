use colored::*;
use sem::{
    expense::Expense,
    stats::{generate_stats, recent_transactions},
    utils::{clear_console, init_source_file, source_file_path},
};
use std::{io, process};
fn main() {
    clear_console();

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

    loop {
        println!();
        println!("ℹ️  {}", "Please select an option:".bright_blue().bold());
        println!("1. Add Expense");
        println!("2. Stats");
        println!("3. Exit");

        // Handle User Input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                match Expense::new() {
                    Ok(expense) => {
                        println!("Expense added {:#?}", expense);
                    }
                    Err(e) => {
                        // Handle the error case
                        println!("{}", e);
                    }
                }
            }
            "2" => generate_stats(),
            "3" => {
                println!("Goodbye 👋 👋");
                process::exit(0);
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
