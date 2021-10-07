use log::*;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Write};

const DB_FILE: &str = "./resources/db.json";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Resident {
    pub id: u32,
    pub name: String,
    pub parking_spots: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Db {
    pub residents: Vec<Resident>,
}

pub fn get_all_residents() -> Result<Vec<Resident>, Box<dyn Error>> {
    info!("get_all_residents...");
    let db: Db = open_db()?;
    Ok(db.residents)
}

pub fn get_resident(id: u32) -> Result<Option<Resident>, Box<dyn Error>> {
    info!("get_residents: {}...", id);
    let db: Db = open_db()?;

    let mut resident: Option<Resident> = None;

    for r in db.residents.into_iter() {
        if r.id == id {
            resident = Some(r);
            break;
        }
    }

    Ok(resident)
}

pub fn insert_residents(mut residents: Vec<Resident>) -> Result<Vec<Resident>, Box<dyn Error>> {
    info!("insert_residents...");
    let mut db: Db = open_db()?;
    db.residents.append(&mut residents);
    write_db(&db)?;
    Ok(db.residents)
}

fn write_db(db: &Db) -> Result<(), Box<dyn Error>> {
    info!("write_db...");
    let mut file = File::create(DB_FILE)?;
    let db_content = serde_json::to_vec_pretty(&db).unwrap();
    file.write_all(db_content.as_slice())?;
    Ok(())
}

fn open_db() -> Result<Db, Box<dyn Error>> {
    info!("open_db...");
    // Open the file or create and initialize it if not found
    let file = match File::open(DB_FILE) {
        Ok(f) => Ok(f),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            warn!("DB file {} not found, create it...", DB_FILE);
            let db = Db::default();
            let mut file = File::create(DB_FILE)?;
            let db_content = serde_json::to_vec_pretty(&db).unwrap();
            file.write_all(db_content.as_slice())?;
            File::open(DB_FILE)
        }
        Err(e) => Err(e),
    }?;
    let reader = BufReader::new(file);
    // Read the JSON contents of the file as an instance of `Db`.
    let db = serde_json::from_reader(reader)?;
    Ok(db)
}
