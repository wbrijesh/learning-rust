use chrono::{Local, NaiveDate};
use std::io;

fn main() {
    println!("Age Calculator");
   
    println!("Enter your birthdate (in DD/MM/YYYY): ");
    let birthdate_input: String = get_input();
    let birthdate: NaiveDate = NaiveDate::parse_from_str(&birthdate_input, "%d/%m/%Y").unwrap();

    let today: NaiveDate = Local::now().date_naive();
    let total_days: u16 = (today - birthdate).num_days().try_into().unwrap();

    let years: u16 = total_days / 365;
    let months: u16 = (total_days % 365) / 30;
    let days: u16 = (total_days % 365) % 30;

    println!(
        "You are {} years, {} months, {} days \n(not accounting for leap years)",
        years, months, days
    );
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("FAILED TO READ INPUT");
    return input.trim().to_string();
}
