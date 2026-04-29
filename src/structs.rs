use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityLocation {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}
