use std::error::Error;

use crate::{
    api::client,
    sql::CityLocationDatabase,
    structs::{CityLocation, Nominatim},
};

async fn fetch_city_location(city_name: &str) -> Result<CityLocation, Box<dyn Error>> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?format=json&q={}&limit=1&accept-language=en",
        city_name
    );

    let data = client::CLIENT
        .get(&url)
        .send()
        .await?
        .json::<Vec<Nominatim>>()
        .await?;

    let city = data.first().ok_or("Could not get first city")?;

    let city_location = CityLocation {
        name: city.name.clone(),
        lat: city.latitude.parse::<f64>()?,
        lon: city.longitude.parse::<f64>()?,
    };

    Ok(city_location)
}

pub async fn get_city_location(city_name: &str) -> Result<CityLocation, Box<dyn Error>> {
    let db = CityLocationDatabase::load_or_create("cities.db")?;

    if let Some(city) = db.get_city(city_name)? {
        return Ok(city);
    }

    let city = fetch_city_location(city_name).await?;
    db.add_city(&city)?;
    Ok(city)
}
