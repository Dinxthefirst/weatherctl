use std::{error::Error, io};

mod api;
mod sql;
mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    loop {
        println!("Enter city name (or 'q' to quit):");

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let city_name = input.trim();

        if city_name.eq_ignore_ascii_case("q") {
            break;
        }

        if city_name.is_empty() {
            continue;
        }

        let city_location = api::city::get_city_location(city_name).await?;
        let city_weather = api::weather::fetch_current_temperature(&city_location).await?;
        println!(
            "It is currently {}{} in {}",
            city_weather.temperature, city_weather.unit, city_weather.name
        );
    }

    Ok(())
}
