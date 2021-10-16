use rocket::request::{FromRequest, Outcome};
use rocket::{fs::NamedFile, response::status::NotFound};
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    serde::json::Json,
    Request, Response,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub mod login;
pub mod residents;

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

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let file = match file.to_str() {
        Some("") => PathBuf::from("index.html"),
        Some(s) if s.starts_with("app") => PathBuf::from("index.html"),
        _ => file,
    };

    let path = Path::new("resources/web-app/").join(file);

    info!("The file requested is {}", path.display());

    NamedFile::open(&path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}
