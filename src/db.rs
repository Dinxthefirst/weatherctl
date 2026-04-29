use crate::structs::CityLocation;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct CityLocationDatabase {
    cities: Vec<CityLocation>,
}

impl CityLocationDatabase {
    pub fn load_or_create(path: &str) -> Result<Self, Box<dyn Error>> {
        if std::path::Path::new(path).exists() {
            let content = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(CityLocationDatabase { cities: Vec::new() })
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn get_city(&self, name: &str) -> Option<&CityLocation> {
        self.cities.iter().find(|city| city.name == name)
    }

    pub fn add_city(&mut self, city_location: CityLocation) {
        if self.get_city(&city_location.name).is_none() {
            self.cities.push(city_location);
        }
    }
}
