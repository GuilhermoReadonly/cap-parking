#[macro_use]
extern crate rocket;

use rocket::response::{Responder, Response};
use rocket::{
    http::{ContentType, Status},
    response,
    serde::json::Json,
    Request,
};
use serde::{Deserialize, Serialize};

use crate::routes::index;
use crate::routes::residents::get_residents;

pub mod routes;

#[derive(Debug, Serialize, Deserialize)]
pub enum Body<T> {
    Ok(T),
    Err(String),
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    status: Status,
    body: Body<Json<T>>,
}

impl<T> ApiResponse<T> {
    pub fn new(body: Body<Json<T>>, status: Status) -> Self {
        ApiResponse { status, body }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'static> {
        match self.body {
            Body::Ok(t) => Response::build_from(t.respond_to(&req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .ok(),
            Body::Err(msg) => Response::build_from(msg.respond_to(&req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .ok(),
        }
    }
}

#[launch]
fn rocket() -> _ {
    info!("Starting app...");

    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![get_residents])
}
