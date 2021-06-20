#[macro_use] extern crate rocket;

use rocket::{serde::{json::Json}};
use cap_parking_lib::{Resident, get_residents};


#[get("/residents")]
fn residents() -> Json<Vec<Resident>> {
    info!("Get residents...");

    let residents = get_residents().unwrap_or(vec![]);


    Json(residents)
}

#[launch]
fn rocket() -> _ {

    info!("Starting app...");
    
    rocket::build().mount("/", routes![residents])
}
