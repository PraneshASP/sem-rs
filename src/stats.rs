use crate::expense::Expense;
use crate::utils::{format_inr, month_to_name, read_expenses_from_csv, source_file_path};
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use colored::Colorize;
use comfy_table::{
    presets::UTF8_FULL, Attribute::Bold, Cell, Color, ContentArrangement, Row, Table,
};
use dialoguer::FuzzySelect;
use dialoguer::{theme::ColorfulTheme, Input};
use std::collections::{BTreeMap, HashMap};

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
    header.add_cell(Cell::new("Category").add_attribute(Bold).fg(Color::Yellow));
    for month in total_per_month.keys() {
        // Convert month number to month name
        let month_name = month_to_name(month);
        header.add_cell(Cell::new(&month_name).add_attribute(Bold));
    }
    header.add_cell(Cell::new("Total").add_attribute(Bold).fg(Color::Blue));
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

            row.add_cell(Cell::new(&format_inr(*amount)).fg(amount_color));
        }
        row.add_cell(Cell::new(format_inr(total_for_category)).fg(Color::Cyan));
        table.add_row(row);
    }

    // Footer Row for Totals
    let mut footer = Row::new();
    footer.add_cell(Cell::new("Total").add_attribute(Bold));
    let mut grand_total = 0.0;
    for total in total_per_month.values() {
        grand_total += total;
        footer.add_cell(Cell::new(format_inr(total.clone())).fg(Color::Cyan));
    }
    footer.add_cell(
        Cell::new(format_inr(grand_total))
            .add_attribute(Bold)
            .fg(Color::DarkCyan),
    );
    table.add_row(footer);

    println!("{}", table);
}

pub fn recent_transactions() {
    let expenses: &[Expense] = &read_expenses_from_csv(source_file_path()).unwrap();
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Add the table headers
    table.set_header(vec![
        Cell::new("Date").add_attribute(Bold),
        Cell::new("Amount").add_attribute(Bold),
        Cell::new("Category").add_attribute(Bold),
        Cell::new("Notes").add_attribute(Bold),
    ]);

    // Add the last five transactions to the table
    for expense in expenses.iter().rev().take(5) {
        table.add_row(Row::from(vec![
            Cell::new(&expense.date),
            Cell::new(format_inr(expense.amount)),
            Cell::new(&expense.category),
            Cell::new(&expense.notes),
        ]));
    }

    // Print the table
    println!("{}", table);
}

fn display_month_breakdown(expenses: &[Expense], month: &str) {
    let mut expenses_by_week: HashMap<u32, f32> = HashMap::new();
    let mut week_offset = 0;

    // Determine the first week number of the month
    if let Ok(first_day_of_month) = NaiveDate::parse_from_str(&format!("{}-01", month), "%Y-%m-%d")
    {
        week_offset = first_day_of_month.iso_week().week();
    }

    // Filter and aggregate expenses by week
    for expense in expenses {
        if let Ok(date) = NaiveDate::parse_from_str(&expense.date, "%Y-%m-%d") {
            if date.format("%Y-%m").to_string() == month {
                let week = date.iso_week().week();
                let relative_week = week.saturating_sub(week_offset) + 1; // Calculate relative week number
                *expenses_by_week.entry(relative_week).or_default() += expense.amount;
            }
        }
    }

    // Creating and setting up the table
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Week").add_attribute(Bold),
        Cell::new("Total Amount").add_attribute(Bold),
    ]);

    // Sorting weeks and populating the table with data
    let mut weeks: Vec<_> = expenses_by_week.keys().collect();
    weeks.sort(); // Sort the weeks
    for &week in weeks {
        let total_amount = expenses_by_week[&week];
        table.add_row(Row::from(vec![
            Cell::new(format!("Week {}", week - 1)),
            Cell::new(format!("{:.2}", total_amount)),
        ]));
    }

    // Print the table
    println!("{}", table);
}

fn display_transactions_for_date(expenses: &[Expense], date: &str) {
    let mut daily_expenses: Vec<&Expense> = Vec::new();

    // Filter expenses for the given date
    for expense in expenses {
        if expense.date == date {
            daily_expenses.push(expense);
        }
    }

    // Creating and setting up the table
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Category").add_attribute(Bold),
        Cell::new("Amount").add_attribute(Bold),
        Cell::new("Notes").add_attribute(Bold),
    ]);

    // Populating the table with data
    for expense in daily_expenses {
        table.add_row(Row::from(vec![
            Cell::new(&expense.category),
            Cell::new(format!("{:.2}", expense.amount)),
            Cell::new(&expense.notes),
        ]));
    }

    // Print the table
    println!("{}", table);
}

// Calculate the start and end dates of the week within the given month
fn get_week_date_range(year: i32, month: u32, week: u32) -> Option<(NaiveDate, NaiveDate)> {
    let first_of_month = NaiveDate::from_ymd_opt(year, month, 1)?;

    // Find the first Monday of the month or the 1st if it's a Monday
    let first_monday = if first_of_month.weekday() == Weekday::Mon {
        first_of_month
    } else {
        first_of_month
            + Duration::days((7 - first_of_month.weekday().num_days_from_monday()) as i64)
    };

    // Calculate the start date of the specified week
    let start_date = first_monday + Duration::days((week as i64 - 1) * 7);
    if start_date.month() != month {
        return None; // The specified week does not exist in the given month
    }

    Some((start_date, start_date + Duration::days(6)))
}

fn parse_week_input(week_input: &str) -> (i32, u32, u32) {
    let parts: Vec<&str> = week_input.split(':').collect();
    let date_parts: Vec<&str> = parts[0].split('-').collect();

    let year = date_parts[0].parse::<i32>().unwrap();
    let month = date_parts[1].parse::<u32>().unwrap();
    let week = parts[1].parse::<u32>().unwrap();

    (year, month, week)
}

fn calculate_weekly_total(expenses: &[Expense], week_input: &str) {
    let (year, month, week) = parse_week_input(week_input);

    if let Some((start_date, end_date)) = get_week_date_range(year, month, week) {
        let mut daily_totals: HashMap<String, f32> = HashMap::new();
        let mut days_of_week: Vec<String> = Vec::new();

        let mut current = start_date;
        while current <= end_date {
            let day_name = current.format("%A").to_string();
            days_of_week.push(day_name.clone());
            let total_amount = expenses
                .iter()
                .filter(|e| e.date.starts_with(&current.format("%Y-%m-%d").to_string()))
                .map(|e| e.amount)
                .sum();
            daily_totals.insert(day_name, total_amount);
            current += Duration::days(1);
        }

        // Creating and setting up the table
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(vec![
            Cell::new("Day").add_attribute(Bold),
            Cell::new("Total Amount").add_attribute(Bold),
        ]);

        // Populating the table with sorted data
        for day in days_of_week {
            if let Some(&amount) = daily_totals.get(&day) {
                table.add_row(Row::from(vec![
                    Cell::new(day),
                    Cell::new(format!("{:.2}", amount)),
                ]));
            }
        }

        // Print the table
        println!("{}", table);
    } else {
        println!("Invalid week number for the given month and year.");
    }
}

pub fn generate_stats() {
    let expenses: &[Expense] = &read_expenses_from_csv(source_file_path()).unwrap();
    let commands = vec!["Total Expenses", "Summary", "Analyze"];
    let command = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select from list: ")
        .default(0)
        .items(&commands)
        .interact()
        .unwrap();

    match command {
        0 => println!(
            "\nTotal expenses: {:>}\n",
            format_inr(calculate_total_expenses(expenses))
                .to_string()
                .bold()
        ),
        1 => display_spending_by_category_table(&expenses),
        2 => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the date/week/month to analyze:")
                .default("2023-05:4".to_string())
                .interact_text()
                .unwrap();

            if input.contains('-') && input.contains(':') {
                calculate_weekly_total(expenses, &input)
            } else if input.len() == 7 {
                display_month_breakdown(expenses, &input)
            } else {
                display_transactions_for_date(expenses, &input)
            }
        }
        _ => println!("Unknown command"),
    }
}
