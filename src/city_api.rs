use std::error::Error;

use crate::db::CityLocationDatabase;
use crate::structs::CityLocation;
use crate::structs::Nominatim;

async fn fetch_city_location(city_name: &str) -> Result<CityLocation, Box<dyn Error>> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?format=json&q={}&limit=1",
        city_name
    );
    println!("{}", url);

    // Needed for nominatim policy
    let client = reqwest::Client::builder()
        .user_agent("rust-weather-app/0.1.0")
        .build()?;

    let data = client
        .get(&url)
        .send()
        .await?
        .json::<Vec<Nominatim>>()
        .await?;

    let city = data.first().ok_or("Could not get first city")?;

    let lat: f64 = city.lat.parse::<f64>()?;
    let lon: f64 = city.lon.parse::<f64>()?;
    let city_name: String = city.name.clone();

    let city_location = CityLocation {
        name: city_name,
        lat: lat,
        lon: lon,
    };

    Ok(city_location)
}

pub async fn get_city_location(city_name: &str) -> Result<CityLocation, Box<dyn Error>> {
    let database_path = "db.json";
    let mut city_location_database: CityLocationDatabase =
        CityLocationDatabase::load_or_create(database_path)?;

    let city_location: &CityLocation = match city_location_database.get_city(city_name) {
        std::prelude::v1::None => {
            let city_location: CityLocation = fetch_city_location(city_name).await?;
            city_location_database.add_city(city_location.clone());
            city_location_database.get_city(city_name).unwrap()
        }
        Some(city_location) => city_location,
    };

    city_location_database.save(database_path)?;

    Ok(city_location.clone())
}
