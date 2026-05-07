use serde::Deserialize;
#[derive(Debug, Clone)]
pub struct CityLocation {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone)]
pub struct CityWeather {
    pub name: String,
    pub temperature: f64,
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct Nominatim {
    #[serde(rename = "lat")]
    pub latitude: String,
    #[serde(rename = "lon")]
    pub longitude: String,
    pub name: String,
    // pub display_name: String,
    // #[serde(rename = "type")]
    // pub result_type: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenMeteo {
    // pub latitude: f64,
    // pub longitude: f64,
    // pub timezone: String,
    #[serde(rename = "current_units")]
    pub unit: CurrentUnits,
    pub current: Current,
}

#[derive(Debug, Deserialize)]
pub struct CurrentUnits {
    #[serde(rename = "temperature_2m")]
    pub temperature_units: String,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    #[serde(rename = "temperature_2m")]
    pub temperature: f64,
}
