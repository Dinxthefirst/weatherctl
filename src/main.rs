use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Nominatim {
    lat: String,
    lon: String,
    name: String,
    display_name: String,
    #[serde(rename = "type")]
    result_type: String,
}

#[derive(Debug)]
struct CityLocation {
    name: String,
    lat: f64,
    lon: f64,
}

async fn city(city_name: &str) -> Result<CityLocation, Box<dyn Error>> {
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
    let city_name = "Copenhagen";
    match city(city_name).await {
        Ok(CityLocation { name, lat, lon }) => {
            println!("name: {}\nlat: {}\nlon: {}", name, lat, lon)
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
