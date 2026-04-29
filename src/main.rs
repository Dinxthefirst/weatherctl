use serde::{Deserialize, Serialize};
use std::error::Error;

mod structs;
use crate::structs::CityLocation;

mod db;
use crate::db::CityLocationDatabase;

#[derive(Debug, Serialize, Deserialize)]
struct Nominatim {
    lat: String,
    lon: String,
    name: String,
    display_name: String,
    #[serde(rename = "type")]
    result_type: String,
}

async fn get_city_location(city_name: &str) -> Result<CityLocation, Box<dyn Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_path = "db.json";
    let mut db: CityLocationDatabase = CityLocationDatabase::load_or_create(db_path)?;

    let city_name = "Los Angeles";
    let city_location: &CityLocation = match db.get_city(city_name) {
        None => {
            let city_location: CityLocation = get_city_location(city_name).await?;
            db.add_city(city_location.clone());
            db.get_city(city_name).unwrap()
        }
        Some(city_location) => city_location,
    };

    println!(
        "name: {}\nlat: {}\nlon: {}",
        city_location.name, city_location.lat, city_location.lon
    );

    db.save(db_path)?;
    Ok(())
}
