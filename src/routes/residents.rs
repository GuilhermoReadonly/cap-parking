use cap_parking_lib::Resident;
use rocket::{http::Status, serde::json::Json};

use crate::{ApiResponse, Body};

#[get("/residents")]
pub fn get_residents() -> ApiResponse<Vec<Resident>> {
    info!("Get residents...");

    match cap_parking_lib::get_all_residents() {
        Ok(residents) => ApiResponse::new(Body::Ok(Json(residents)), Status::Ok),
        Err(e) => {
            error!("{}", e);
            ApiResponse::new(
                Body::Err(format!("{{\"error\": \"{}\"}}", e)),
                Status::ImATeapot,
            )
        }
    }
}

#[put("/resident", data = "<resident>")]
pub fn put_resident(resident: Json<Resident>) -> ApiResponse<Vec<Resident>> {
    info!("Put residents...");

    let resident = resident.0;

    match cap_parking_lib::insert_residents(vec![resident]) {
        Ok(residents) => ApiResponse::new(Body::Ok(Json(residents)), Status::Ok),
        Err(e) => {
            error!("{}", e);
            ApiResponse::new(
                Body::Err(format!("{{\"error\": \"{}\"}}", e)),
                Status::ImATeapot,
            )
        }
    }
}
