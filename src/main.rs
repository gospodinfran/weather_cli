use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::{Error as SerdeJsonError, Value};
use std::fmt;
use std::io;

#[derive(Deserialize, Debug)]
struct Weather {
    temp_c: f64,
    feelslike_c: f64,
    condition: Condition,
}

#[derive(Deserialize, Debug)]
struct Condition {
    text: String,
}

#[derive(Debug)]
enum CustomError {
    Reqwest(ReqwestError),
    SerdeJson(SerdeJsonError),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Reqwest(e) => e.fmt(f),
            CustomError::SerdeJson(e) => e.fmt(f),
        }
    }
}

impl From<ReqwestError> for CustomError {
    fn from(error: ReqwestError) -> Self {
        CustomError::Reqwest(error)
    }
}

impl From<SerdeJsonError> for CustomError {
    fn from(error: SerdeJsonError) -> Self {
        CustomError::SerdeJson(error)
    }
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let api_key = "YOUR_API_KEY";

    let mut location = String::new();

    println!("Enter a city to get its weather report!");

    io::stdin()
        .read_line(&mut location)
        .expect("Invalid location.");

    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}",
        api_key, location
    );

    let response: Value = reqwest::get(&url).await?.json().await?;
    let weather: Weather = serde_json::from_value(response["current"].clone())?;

    let mut message = format!(
        "The current temperature in {} is {}°C and the weather is {}.",
        location,
        weather.temp_c,
        weather.condition.text.to_lowercase()
    );

    if (weather.temp_c - weather.feelslike_c).abs() >= 5.0 {
        message = format!(
            "{} But be warned that it feels more like {}.",
            message, weather.feelslike_c
        );
    }

    println!("{}", message);

    Ok(())
}
