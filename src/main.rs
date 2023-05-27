use std::env;
use chrono::prelude::*;

#[derive(Debug)]
struct Config {
    amount_of_money: u32,
    currency: String,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        let amount_of_money = args[1].parse().unwrap();
        let currency = args[2].clone();

        Config {
            currency,
            amount_of_money
        }
    }
}

struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl Date {
    fn new(date: DateTime<Utc>) -> Date {
        Date {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }
}

fn main() {
    let config = Config::new(env::args().collect());

    println!("amount_of_money: {}", config.amount_of_money);
    println!("currency: {}", config.currency);

    let current_date: Date = Date::new(Utc::now());

    println!("year: {}", current_date.year);
    println!("month: {}", current_date.month);
    println!("day: {}", current_date.day);

}
