use colored::*;
use sem::{expense::Expense, stats::generate_stats};
use std::{io, process};

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

    println!();

    loop {
        println!();
        println!("ℹ️ {}", "Please select an option:".bright_blue().bold());
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
