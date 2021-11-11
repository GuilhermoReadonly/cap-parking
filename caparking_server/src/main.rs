#[macro_use]
extern crate rocket;
use std::env;

use rand::{distributions::Alphanumeric, Rng};

use crate::routes::{
    files,
    login::login,
    residents::{get_resident, get_residents, put_resident},
};

mod guards;
mod routes;

pub struct Secret(String);

#[launch]
fn rocket() -> _ {
    info!("Generate secret...");

    let secret: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(42)
        .map(char::from)
        .collect();

    dbg!(&secret);

    let secret = Secret(secret);

    let cwd = env::current_dir().expect("Can't get current directory.");
    info!("The current directory is {}", cwd.display());

    info!("Starting app...");
    rocket::build()
        .mount("/", routes![files])
        .mount(
            "/api",
            routes![login, get_residents, get_resident, put_resident],
        )
        .manage(secret)
}
