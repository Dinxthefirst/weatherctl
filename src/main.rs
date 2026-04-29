use std::error::Error;

mod api;
mod db;
mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let city_name = "Tokyo";

    let city_location: structs::CityLocation = api::city::get_city_location(city_name).await?;

    // println!(
    //     "name: {}\nlat: {}\nlon: {}",
    //     city_location.name, city_location.lat, city_location.lon
    // );

    let city_weather: structs::CityWeather =
        api::weather::fetch_current_temperature(city_location).await?;

    println!(
        "It is currently {}{} in {}",
        city_weather.temperature, city_weather.unit, city_weather.name
    );

    Ok(())
}
