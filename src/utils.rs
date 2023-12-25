use crate::expense::Expense;
use chrono::NaiveDate;
use console::Term;
use csv::Reader;
use csv::WriterBuilder;
use std::env;
use std::error::Error;
use std::fs::File;
use std::fs::{self};

pub fn read_expenses_from_csv(file_path: &str) -> Result<Vec<Expense>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);
    let mut expenses = Vec::new();

    for result in rdr.deserialize() {
        let record: Expense = result?;
        expenses.push(record);
    }

    Ok(expenses)
}

pub fn write_transactions_to_csv(transactions: &[Expense]) -> Result<(), csv::Error> {
    let file = File::create("expenses.csv")?;
    let mut wtr = WriterBuilder::new().from_writer(file);

    for transaction in transactions {
        wtr.serialize(transaction)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn month_to_name(month_str: &str) -> String {
    // Parse the string to a date
    if let Ok(date) = NaiveDate::parse_from_str(&format!("{}-01", month_str), "%Y-%m-%d") {
        date.format("%B").to_string() // %B formats the date to full month name
    } else {
        month_str.to_string() // In case of parsing error, return the original string
    }
}

pub fn format_inr(num: f32) -> String {
    let inr = num as i64;
    let mut inr_str = inr.to_string();
    let len = inr_str.len();

    if len > 3 {
        inr_str.insert(len - 3, ',');
        let mut pos = len - 3;
        while pos > 2 {
            pos -= 2;
            inr_str.insert(pos, ',');
        }
    }

    // Handle decimal part
    let decimals = (num.fract() * 100.0).round() / 100.0; // Adjust for desired decimal places
    format!(
        "{}.{}",
        inr_str,
        format!("{:.2}", decimals).split('.').nth(1).unwrap()
    )
}

pub fn clear_console() {
    let term = Term::stdout();
    term.clear_screen()
        .unwrap_or_else(|e| eprintln!("Failed to clear screen: {}", e));
}
