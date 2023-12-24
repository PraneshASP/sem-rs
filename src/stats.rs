use crate::expense::Expense;
use chrono::NaiveDate;
use colored::*;
use std::collections::{BTreeMap, HashMap};
use std::io;

fn calculate_total_expenses(expenses: &[Expense]) -> f32 {
    expenses.iter().map(|e| e.amount).sum()
}

fn prepare_spending_data(
    expenses: &[Expense],
) -> (
    HashMap<String, BTreeMap<String, f32>>,
    BTreeMap<String, f32>,
    HashMap<String, f32>,
) {
    let mut data_by_category_and_month: HashMap<String, BTreeMap<String, f32>> = HashMap::new();
    let mut total_per_month: BTreeMap<String, f32> = BTreeMap::new();
    let mut total_per_category: HashMap<String, f32> = HashMap::new();

    for expense in expenses {
        let date = NaiveDate::parse_from_str(&expense.date, "%Y-%m-%d").unwrap();
        let month = date.format("%Y-%m").to_string();
        *total_per_month.entry(month.clone()).or_insert(0.0) += expense.amount;

        let category_totals = data_by_category_and_month
            .entry(expense.category.clone())
            .or_insert_with(BTreeMap::new);
        *category_totals.entry(month.clone()).or_insert(0.0) += expense.amount;
        *total_per_category
            .entry(expense.category.clone())
            .or_insert(0.0) += expense.amount;
    }

    (
        data_by_category_and_month,
        total_per_month,
        total_per_category,
    )
}

fn display_spending_by_category_table(expenses: &[Expense]) {
    let (data, total_per_month, total_per_category) = prepare_spending_data(expenses);
    println!();

    // Print table header with months
    print!("{:<15}", "Category".bold().yellow());
    for month in total_per_month.keys() {
        print!("{:>10}", month);
    }
    println!("{:>10}", "Total".blue());

    println!("{}", "-".repeat(15 + 10 * (total_per_month.len() + 1)));

    // Print each category and its monthly totals
    for (category, monthly_totals) in &data {
        print!("{:<15}", category.yellow());
        let mut total_for_category = 0.0;
        for month in total_per_month.keys() {
            let amount = monthly_totals.get(month).unwrap_or(&0.0);
            total_for_category += amount;
            let amount_color = if *amount < 6000.0 {
                amount.to_string().green()
            } else {
                amount.to_string().red()
            };

            print!("{:>10}", amount_color);
        }
        println!("{:>10.10}", total_for_category.to_string().cyan());
    }

    // Print the final row with totals for each month
    print!("{:<15}", "Total");
    let mut grand_total = 0.0;
    for total in total_per_month.values() {
        grand_total += total;
        print!("{:>10}", total.to_string().cyan());
    }
    println!("{:>10}", grand_total.to_string().bold().bright_cyan());
    println!();
}

pub fn generate_stats(expenses: &[Expense]) {
    println!("Available commands: total, summary");
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    match command.trim() {
        "total" => println!("Total expenses: {:.2}", calculate_total_expenses(expenses)),

        "summary" => display_spending_by_category_table(&expenses),
        _ => println!("Unknown command"),
    }
}
