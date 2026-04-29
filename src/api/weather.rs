// latitude 52.52
// longitude 13.41
// always celsius unless &temperature_unit=fahrenheit
// https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m&past_days=0&forecast_days=1
use std::error::Error;

use crate::structs::{CityLocation, CityWeather, OpenMeteo};

pub async fn fetch_current_temperature(
    city_location: CityLocation,
) -> Result<CityWeather, Box<dyn Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m&past_days=0&forecast_days=1",
        city_location.lat, city_location.lon,
    );
    // println!("weather url: {}", url);

    let client = reqwest::Client::builder()
        .user_agent("rust-weather-app/0.1.0")
        .build()?;

    let open_meteo: OpenMeteo = client.get(&url).send().await?.json::<OpenMeteo>().await?;

    // println!(
    //     "weather data:\nlat: {}\nlon: {}\ntemp: {}\nunit: {}",
    //     open_meteo.latitude,
    //     open_meteo.longitude,
    //     open_meteo.temperature.temperature,
    //     open_meteo.unit.unit,
    // );

    let city_weather: CityWeather = CityWeather {
        name: city_location.name,
        temperature: open_meteo.temperature.temperature,
        unit: open_meteo.unit.unit,
    };

    Ok(city_weather)
}
