use std::error::Error;

mod city_api;
mod db;
mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let city_name = "Odense";

    let city_location: structs::CityLocation = city_api::get_city_location(city_name).await?;

    println!(
        "name: {}\nlat: {}\nlon: {}",
        city_location.name, city_location.lat, city_location.lon
    );

    Ok(())
}
