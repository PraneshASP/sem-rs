use crate::expense::Expense;
use chrono::{NaiveDate, Utc};
use console::Term;
use csv::{Reader, WriterBuilder};
use directories::ProjectDirs;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub fn read_expenses_from_csv(file_path: PathBuf) -> Result<Vec<Expense>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);
    let mut expenses = Vec::new();
    for result in rdr.deserialize() {
        match result {
            Ok(record) => expenses.push(record),
            Err(e) => eprintln!("Error reading CSV record: {}", e),
        }
    }

    Ok(expenses)
}

pub fn write_transactions_to_csv(transactions: &[Expense]) -> Result<(), csv::Error> {
    let file = File::create(source_file_path())?;
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

pub fn source_file_path() -> PathBuf {
    let current_year = Utc::now().format("%Y").to_string();
    let filename = format!("expenses-{}.csv", current_year);

    let project_dirs = ProjectDirs::from("", "", "sem-rs").expect("Cannot find home directory");
    project_dirs.data_local_dir().join(filename)
}

pub fn init_source_file(file_path: &PathBuf) {
    let dir = file_path.parent().expect("Failed to find parent directory");

    if !dir.exists() {
        fs::create_dir_all(dir).expect("Failed to create directory");
    }

    if !file_path.exists() {
        let mut file = File::create(file_path).expect("Failed to create file");
        writeln!(file, "date,amount,category,notes").expect("Failed to write headers");
    }
}
