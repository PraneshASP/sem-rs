use dialoguer::theme::ColorfulTheme;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

use chrono::Local;
use dialoguer::{Confirm, FuzzySelect, Input, Select};
use strum::IntoEnumIterator;

use csv::{Writer, WriterBuilder};
use std::error::Error;
use std::{fs::OpenOptions, path::Path};

use chrono::{Duration, NaiveDate};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub date: String,
    pub amount: f32,
    pub category: String,
    pub notes: String,
}

#[derive(EnumString, EnumIter, Display, AsRefStr, Debug)]
pub enum Categories {
    Fashion,
    Food,
    Car,
    Misc,
    Groceries,
    Housing,
}

impl Expense {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let today = Local::now().format("%Y-%m-%d").to_string();

        let date: String = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the date of the expense")
            .default(today)
            .interact_text()
            .unwrap();

        let amount_str: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the amount")
            .interact_text()
            .unwrap();
        let amount: f32 = amount_str.parse().expect("Please enter a valid number");

        let categories: Vec<String> = Categories::iter().map(|c| c.as_ref().to_string()).collect();
        let category_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a category")
            .default(0)
            .items(&categories)
            .interact()
            .unwrap();

        let category = Categories::iter().nth(category_selection).unwrap();

        let notes: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter any notes")
            .default(" ".to_string())
            .interact_text()
            .unwrap();

        let expense = Expense {
            date,
            amount,
            category: category.to_string(),
            notes,
        };

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to save this expense?")
            .interact()
            .unwrap()
        {
            let file_path = "expenses.csv";
            let file_exists = Path::new(file_path).exists();

            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)?;

            let mut wtr = WriterBuilder::new()
                .has_headers(!file_exists)
                .from_writer(file);

            if !file_exists {
                // If the file did not exist, write the headers
                wtr.write_record(&["Date", "Amount", "Category", "Notes"])?;
            }

            wtr.serialize(&expense)?;
            wtr.flush()?;
            Ok(expense)
        } else {
            return Err("Expense not saved".into());
        }
    }

    pub fn generate_transactions() -> Vec<Expense> {
        let mut rng = thread_rng();
        let categories = vec![
            "Food",
            "Transport",
            "Housing",
            "Utilities",
            "Entertainment",
            "Misc",
        ];
        let date_range = Uniform::new(0, 212);
        let mut expenses = Vec::new();

        for _ in 0..1000 {
            let random_days = rng.sample(date_range);
            let random_date = NaiveDate::from_ymd(2023, 1, 1) + Duration::days(random_days.into());
            let random_category = categories[rng.gen_range(0..categories.len())];
            let random_amount = rng.gen_range(10.0..500.0);
            let random_notes = "Generated Note";

            expenses.push(Expense {
                date: random_date.format("%Y-%m-%d").to_string(),
                amount: random_amount,
                category: random_category.to_string(),
                notes: random_notes.to_string(),
            });
        }

        expenses
    }
}
