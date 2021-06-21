use log::*;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct Resident {
    pub id: u32,
    pub name: String,
    pub parking_spots: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Db {
    pub residents: Vec<Resident>,
}

pub fn get_all_residents() -> Result<Vec<Resident>, Box<dyn Error>> {
    info!("Get residents...");

    // Open the file in read-only mode with buffer.
    let file = File::open("./resources/db.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Db`.
    let db: Db = serde_json::from_reader(reader)?;

    Ok(db.residents)
}
