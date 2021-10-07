#[macro_use]
extern crate rocket;
use std::env;

use crate::routes::{
    files,
    residents::{get_resident, get_residents, put_resident},
};

pub mod routes;

#[launch]
fn rocket() -> _ {
    info!("Starting app...");

    let cwd = env::current_dir().expect("yes it is");
    info!("The current directory is {}", cwd.display());

    rocket::build()
        .mount("/", routes![files])
        .mount("/api", routes![get_residents, get_resident, put_resident])
}
