use crate::expense::Expense;
use crate::utils::read_expenses_from_csv;
use chrono::NaiveDate;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, Row, Table};
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

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    // Header Row
    let mut header = Row::new();
    header.add_cell(
        Cell::new("Category")
            .add_attribute(Attribute::Bold)
            .fg(Color::Yellow),
    );
    for month in total_per_month.keys() {
        header.add_cell(Cell::new(month));
    }
    header.add_cell(
        Cell::new("Total")
            .add_attribute(Attribute::Bold)
            .fg(Color::Blue),
    );
    table.set_header(header);

    // Data Rows
    for (category, monthly_totals) in &data {
        let mut row = Row::new();
        row.add_cell(Cell::new(category).fg(Color::Yellow));
        let mut total_for_category = 0.0;
        for month in total_per_month.keys() {
            let amount = monthly_totals.get(month).unwrap_or(&0.0);
            total_for_category += amount;
            let amount_color = if *amount < 6000.0 {
                Color::Green
            } else {
                Color::Red
            };

            row.add_cell(Cell::new(&format!("{:.2}", amount)).fg(amount_color));
        }
        row.add_cell(Cell::new(&format!("{:.2}", total_for_category)).fg(Color::Cyan));
        table.add_row(row);
    }

    // Footer Row for Totals
    let mut footer = Row::new();
    footer.add_cell(Cell::new("Total").add_attribute(Attribute::Bold));
    let mut grand_total = 0.0;
    for total in total_per_month.values() {
        grand_total += total;
        footer.add_cell(Cell::new(&format!("{:.2}", total)).fg(Color::Cyan));
    }
    footer.add_cell(
        Cell::new(&format!("{:.2}", grand_total))
            .add_attribute(Attribute::Bold)
            .fg(Color::DarkCyan),
    );
    table.add_row(footer);

    println!("{}", table);
}

pub fn generate_stats() {
    let expenses: &[Expense] = &read_expenses_from_csv("expenses.csv").unwrap();
    println!("Available commands: total, summary");
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    match command.trim() {
        "total" => println!("Total expenses: {:.2}", calculate_total_expenses(expenses)),
        "summary" => display_spending_by_category_table(&expenses),
        _ => println!("Unknown command"),
    }
}
