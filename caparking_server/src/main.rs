#[macro_use]
extern crate rocket;
use std::env;

use crate::routes::{
    files,
    login::login,
    residents::{get_resident, get_residents, put_resident},
};

pub mod routes;

#[launch]
fn rocket() -> _ {
    info!("Starting app...");

    let cwd = env::current_dir().expect("Can't get current directory.");
    info!("The current directory is {}", cwd.display());

    rocket::build().mount("/", routes![files]).mount(
        "/api",
        routes![login, get_residents, get_resident, put_resident],
    )
}
