use log::*;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Write};

const DB_FILE: &str = "./resources/db.json";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub sub: ResidentSafe, // Optional. Subject (whom token refers to)
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LoginForm {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct ResidentSafe {
    pub id: u128,
    pub name: String,
    pub login: String,
    pub parking_spots: Vec<u32>,
}

impl From<Resident> for ResidentSafe {
    fn from(r: Resident) -> Self {
        Self {
            id: r.id,
            name: r.name,
            login: r.login,
            parking_spots: r.parking_spots,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Resident {
    pub id: u128,
    pub name: String,
    pub login: String,
    pub password: String,
    pub parking_spots: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct ResidentPartial {
    pub id: u128,
    pub name: Option<String>,
    pub login: Option<String>,
    pub password: Option<String>,
    pub parking_spots: Option<Vec<u32>>,
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

pub fn get_resident(id: u128) -> Result<Option<Resident>, Box<dyn Error>> {
    info!("get_resident: {}...", id);
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

pub fn get_resident_by_login(login: String) -> Result<Option<Resident>, Box<dyn Error>> {
    info!("get_resident_by_login: {}...", login);
    let db: Db = open_db()?;

    let mut resident: Option<Resident> = None;

    for r in db.residents.into_iter() {
        if r.login == login {
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

pub fn update_resident(resident_update: ResidentPartial) -> Result<Resident, Box<dyn Error>> {
    info!("update_resident...");
    let mut db: Db = open_db()?;
    let option_found_resident = db.residents.iter_mut().find(|r| r.id == resident_update.id);
    match option_found_resident {
        Some(found_resident) => {
            let updated_resident = Resident {
                id: resident_update.id,
                login: resident_update
                    .login
                    .unwrap_or(found_resident.login.clone()),
                name: resident_update.name.unwrap_or(found_resident.name.clone()),
                password: resident_update
                    .password
                    .unwrap_or(found_resident.password.clone()),
                parking_spots: resident_update
                    .parking_spots
                    .unwrap_or(found_resident.parking_spots.clone()),
            };
            *found_resident = updated_resident.clone();
            write_db(&db)?;
            Ok(updated_resident)
        }
        None => Err(format!("Resident {} not found", resident_update.id).into()),
    }
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
