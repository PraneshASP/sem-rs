use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

use chrono::Local;
use dialoguer::{Confirm, Input, Select};
use strum::IntoEnumIterator;

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
    pub fn new() -> Self {
        let today = Local::now().format("%Y-%m-%d").to_string();

        let date: String = Input::<String>::new()
            .with_prompt("Enter the date of the expense")
            .default(today)
            .interact_text()
            .unwrap();

        let amount_str: String = Input::<String>::new()
            .with_prompt("Enter the amount")
            .interact_text()
            .unwrap();
        let amount: f32 = amount_str.parse().expect("Please enter a valid number");

        let categories: Vec<String> = Categories::iter().map(|c| c.as_ref().to_string()).collect();
        let category_selection = Select::new()
            .with_prompt("Select a category")
            .default(0)
            .items(&categories)
            .interact()
            .unwrap();

        let category = Categories::iter().nth(category_selection).unwrap();

        let notes: String = Input::<String>::new()
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

        if Confirm::new()
            .with_prompt("Do you want to save this expense?")
            .interact()
            .unwrap()
        {
            // Here you can add functionality to save the expense
            expense
        } else {
            panic!("\nExpense not saved.");
        }
    }
}
