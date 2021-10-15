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

pub struct Token<'r> {
    raw_token: &'r str,
}
struct RawToken<'r>(&'r str);

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
    Expired,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn check_token_presence(raw_token: Option<&str>) -> Result<RawToken, TokenError> {
            match raw_token {
                Some(t) => Ok(RawToken(t)),
                None => Err(TokenError::Missing),
            }
        }
        fn decode_raw_token(raw_token: Result<RawToken, TokenError>) -> Result<Token, TokenError> {
            match raw_token {
                Ok(t) => {
                    if t.0.starts_with("718") {
                        Ok(Token { raw_token: t.0 })
                    } else {
                        Err(TokenError::Invalid)
                    }
                }
                Err(e) => Err(e),
            }
        }
        fn validate_token(token: Result<Token, TokenError>) -> Result<Token, TokenError> {
            match token {
                Ok(t) => {
                    if t.raw_token.starts_with("718718") {
                        Ok(t)
                    } else {
                        Err(TokenError::Expired)
                    }
                }
                Err(e) => Err(e),
            }
        }

        let option_raw_token = req.headers().get_one("Authorization");
        let raw_token = check_token_presence(option_raw_token);
        let token_decoded = decode_raw_token(raw_token);
        let token_validated = validate_token(token_decoded);

        match token_validated {
            Ok(t) => Outcome::Success(t),
            Err(token_error) => Outcome::Failure((Status::BadRequest, token_error)),
        }
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
