use rusqlite::{Connection, Result, params};

use crate::structs::CityLocation;

pub struct CityLocationDatabase {
    conn: Connection,
}

impl CityLocationDatabase {
    pub fn load_or_create(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS cities (
                name TEXT PRIMARY KEY,
                lat  REAL NOT NULL,
                lon  REAL NOT NULL
            );
        ",
        )?;
        Ok(Self { conn })
    }

    pub fn get_city(&self, name: &str) -> Result<Option<CityLocation>> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, lat, lon FROM cities WHERE name = ?1")?;
        let mut rows = stmt.query_map(params![name], |row| {
            Ok(CityLocation {
                name: row.get(0)?,
                lat: row.get(1)?,
                lon: row.get(2)?,
            })
        })?;
        rows.next().transpose()
    }

    pub fn add_city(&self, city: &CityLocation) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO cities (name, lat, lon) VALUES (?1, ?2, ?3)",
            params![city.name, city.lat, city.lon],
        )?;
        Ok(())
    }

    // pub fn all_cities(&self) -> Result<Vec<CityLocation>> {
    //     let mut stmt = self.conn.prepare("SELECT name, lat, lon FROM cities")?;
    //     let rows = stmt.query_map([], |row| {
    //         Ok(CityLocation {
    //             name: row.get(0)?,
    //             lat: row.get(1)?,
    //             lon: row.get(2)?,
    //         })
    //     })?;
    //     rows.collect()
    // }
}
