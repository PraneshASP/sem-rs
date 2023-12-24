use csv::Reader;
use csv::{Writer, WriterBuilder};
use std::error::Error;
use std::fs::File;

use crate::expense::Expense;

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
