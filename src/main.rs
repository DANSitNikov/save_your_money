use chrono::prelude::*;
use dotenv::dotenv;
use serde_json::Value;
use std::env;

#[derive(Debug)]
enum Currency {
    USD,
    AMD,
    RUB,
}

#[derive(Debug)]
struct Config {
    amount_of_money: u32,
    currency: Currency,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        let amount_of_money = args[1].parse().unwrap();
        let currency = args[2].clone();

        let currency: Currency = match currency.to_lowercase().as_str() {
            "usd" => Currency::USD,
            "amd" => Currency::AMD,
            "rub" => Currency::RUB,
            _ => panic!("Incorrect currency"),
        };

        Config {
            currency,
            amount_of_money,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = Config::new(env::args().collect());

    println!("amount_of_money: {}", config.amount_of_money);
    println!("currency: {:?}", config.currency);

    let current_date: Date = Date::new(Utc::now());

    println!("year: {}", current_date.year);
    println!("month: {}", current_date.month);
    println!("day: {}", current_date.day);

    let api_key = env::var("CURRENCY_API_KEY").unwrap();

    println!("API: {:?}", api_key);

    let url = format!(
        "http://api.exchangeratesapi.io/v1/latest?access_key={}&symbols=AMD,RUB,USD",
        api_key,
    );

    let resp = reqwest::get(url).await?.text().await?;

    let deserialized: Value = serde_json::from_str(&resp).unwrap();

    println!("{:#?}", deserialized);

    Ok(())
}
