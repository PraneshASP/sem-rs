
# Simple Expense Manager (sem-rs) • [![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org/) [![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)


![](./assets/sem-intro.gif)

`sem-rs` is a simple, command-line tool for managing your personal expenses. Built over a weekend as a personal project, this application is an alternative to using spreadsheets for logging expenses. It works completely offline, storing all data locally.

***Note:** This project was developed quickly over a weekend, so expect some bugs. If you're looking for a more advanced tool, check out [Rex](https://github.com/TheRustyPickle/Rex).*

## Features:
- Easily add and view added expenses.
- Data stored in CSV format, by year.
- Offline functionality – no internet required.
- Current support for figures formatted in INR (Indian Rupee).

## Building from source:

To install Rust Expense Tracker, follow these steps:

1. Ensure you have Rust installed on your system. If not, install it from [here](https://www.rust-lang.org/tools/install).
2. Clone the repository:
```bash
git clone https://github.com/PraneshASP/sem-rs.git
```
3. Navigate to the cloned directory:
```bash
cd sem-rs
```
4. Build project using Cargo:
```bash
cargo build --release
```
5. Run the binary:
```bash
./target/release/sem -h
```

## Source File Format and Location

- **Format:** The application uses CSV files to store expense data.
- **Organization:** Expenses are categorized and stored yearly.

## Sneak Peek:
<details open>
  <summary><b>Adding new expense</b></summary>

![](./assets/sem-new.gif)

</details>
 
<br/>

<details>
  <summary><b>Stats</b></summary>

![](./assets/sem-summary.gif)

</details>

<br/>

**Future Plans:**
- Transition to SQLite for more robust data management.
- Implementation of a user config file for custom settings, including currency format.

## Contributing

Feel free to report bugs or suggest features by opening an issue on the repository.

