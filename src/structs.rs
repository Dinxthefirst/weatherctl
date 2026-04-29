use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityLocation {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityWeather {
    pub name: String,
    pub temperature: f64,
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nominatim {
    // TODO rename lat and lon to full name, and use serde rename to make sure that it is the right objects in the json api call that gets placed at lat and lon location in this object
    pub lat: String,
    pub lon: String,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub result_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenMeteo {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    #[serde(rename = "current_units")]
    pub unit: TemperatureUnit,
    #[serde(rename = "current")]
    pub temperature: Temperature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemperatureUnit {
    #[serde(rename = "temperature_2m")]
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    #[serde(rename = "temperature_2m")]
    pub temperature: f64,
}
