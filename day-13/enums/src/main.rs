use std::time::{Date, Weekday};

#[derive(Debug)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

fn get_next_date_for_day(day: Weekday) -> Weekday {
    return day
}

fn main() {
    let favorite_day = Weekday::Friday;
    println!("day is {:#?}", get_next_date_for_day(favorite_day));
}
