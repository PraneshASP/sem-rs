use colored::*;
use sem::expense::Expense;
use std::{io, process};

fn main() {
    // simple Expense manager
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
        // Display Options
        println!("ℹ️ {}", "Please select an option:".bright_blue().bold());
        println!("1. Add Expense");
        println!("2. Exit");

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
                // Implement the functionality to add an expense
            }
            "2" => {
                println!("Goodbye 👋 👋");
                process::exit(0);
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
