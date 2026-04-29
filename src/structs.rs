use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityLocation {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nominatim {
    pub lat: String,
    pub lon: String,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub result_type: String,
}
