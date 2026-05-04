// latitude 52.52
// longitude 13.41
// always celsius unless &temperature_unit=fahrenheit
// https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m&past_days=0&forecast_days=1
use std::error::Error;

use crate::{
    api::client,
    structs::{CityLocation, CityWeather, OpenMeteo},
};

pub async fn fetch_current_temperature(city: &CityLocation) -> Result<CityWeather, Box<dyn Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m&past_days=0&forecast_days=1",
        city.lat, city.lon,
    );
    // println!("weather url: {}", url);

    let data = client::CLIENT
        .get(&url)
        .send()
        .await?
        .json::<OpenMeteo>()
        .await?;

    // println!(
    //     "weather data:\nlat: {}\nlon: {}\ntemp: {}\nunit: {}",
    //     open_meteo.latitude,
    //     open_meteo.longitude,
    //     open_meteo.temperature.temperature,
    //     open_meteo.unit.unit,
    // );

    Ok(CityWeather {
        name: city.name.clone(),
        temperature: data.temperature.temperature,
        unit: data.unit.unit,
    })
}
