use caparking_lib::{Resident, ResidentPartial, ResidentSafe};
use rocket::{http::Status, serde::json::Json};

use crate::{
    guards::SecurityGuard,
    routes::{ApiResponse, Body},
};

#[get("/residents")]
pub fn get_residents(_token: SecurityGuard) -> ApiResponse<Vec<ResidentSafe>> {
    info!("Get residents...");

    match caparking_lib::get_all_residents() {
        Ok(residents) => {
            let residents_safe = residents.into_iter().map(Into::into).collect();
            ApiResponse::new(Body::Ok(Json(residents_safe)), Status::Ok)
        }
        Err(e) => {
            error!("{}", e);
            ApiResponse::new(
                Body::Err(format!("{{\"error\": \"{}\"}}", e)),
                Status::ImATeapot,
            )
        }
    }
}

#[get("/resident/<id>")]
pub fn get_resident(id: u128, _token: SecurityGuard) -> ApiResponse<ResidentSafe> {
    info!("Get resident {}...", id);

    match caparking_lib::get_resident(id) {
        Ok(Some(resident)) => ApiResponse::new(Body::Ok(Json(resident.into())), Status::Ok),
        Ok(None) => ApiResponse::new(
            Body::Err(format!("{{\"error\": \"resident {} not found\"}}", id)),
            Status::NotFound,
        ),
        Err(e) => {
            error!("{}", e);
            ApiResponse::new(
                Body::Err(format!("{{\"error\": \"{}\"}}", e)),
                Status::ImATeapot,
            )
        }
    }
}

#[post("/resident", data = "<resident>")]
pub fn _post_resident(
    resident: Json<Resident>,
    _token: SecurityGuard,
) -> ApiResponse<Vec<Resident>> {
    info!("Post residents...");

    let resident = resident.0;

    match caparking_lib::insert_residents(vec![resident]) {
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
pub fn put_resident(
    resident: Json<ResidentPartial>,
    token: SecurityGuard,
) -> ApiResponse<Resident> {
    info!("Put residents...");

    let resident = resident.0;

    if resident.id == token.decoded_token.sub.id {
        match caparking_lib::update_resident(resident) {
            Ok(resident) => ApiResponse::new(Body::Ok(Json(resident)), Status::Ok),
            Err(e) => {
                error!("{}", e);
                ApiResponse::new(
                    Body::Err(format!("{{\"error\": \"{}\"}}", e)),
                    Status::ImATeapot,
                )
            }
        }
    } else {
        let e = format!("User {} can't modify other user {}", token.decoded_token.sub.id, resident.id );
        error!("{}", e);
        ApiResponse::new(
            Body::Err(format!("{{\"error\": \"{}\"}}", e)),
            Status::Forbidden,
        )
    }


}
