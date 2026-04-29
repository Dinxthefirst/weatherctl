use std::error::Error;

mod city_api;
mod db;
mod structs;
mod weather_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let city_name = "Tokyo";

    let city_location: structs::CityLocation = city_api::get_city_location(city_name).await?;

    // println!(
    //     "name: {}\nlat: {}\nlon: {}",
    //     city_location.name, city_location.lat, city_location.lon
    // );

    let city_weather: structs::CityWeather =
        weather_api::fetch_current_temperature(city_location).await?;

    println!(
        "It is currently {}{} in {}",
        city_weather.temperature, city_weather.unit, city_weather.name
    );

    Ok(())
}
