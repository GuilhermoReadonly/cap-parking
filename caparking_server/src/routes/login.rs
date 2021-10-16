use crate::{
    routes::{ApiResponse, Body},
    Secret,
};
use caparking_lib::{LoginForm, ResidentSafe, Token};
use chrono::{Duration, Utc};
use rocket::{http::Status, log::private::warn, serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    sub: ResidentSafe, // Optional. Subject (whom token refers to)
}

#[post("/login", data = "<login_form>")]
pub fn login(login_form: Json<LoginForm>, secret: &State<Secret>) -> ApiResponse<Token> {
    info!("login...");

    let resident = caparking_lib::get_resident_by_login(login_form.login.clone());
    let expiration = Utc::now().checked_add_signed(Duration::minutes(10));

    match (resident, expiration) {
        (Ok(Some(r)), Some(exp)) if login_form.password == r.password => {
            let exp = exp.timestamp() as usize;
            let claims = Claims { exp, sub: r.into() };
            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(secret.0.as_ref()),
            );

            match token {
                Ok(t) => ApiResponse::new(Body::Ok(Json(Token { token: t })), Status::Ok),
                Err(e) => {
                    error!("Can't compute token: {}", e);
                    ApiResponse::new(
                        Body::Err(format!(
                            "{{\"error\": \"Something terrible happened: {}\"}}",
                            e
                        )),
                        Status::ImATeapot,
                    )
                }
            }
        }
        (Ok(Some(_)), Some(_)) => {
            warn!("Bad login : bad password");
            ApiResponse::new(
                Body::Err(format!("{{\"error\": \"Bad login\"}}")),
                Status::Unauthorized,
            )
        }
        (Ok(None), _) => {
            warn!("Bad login : bad login");
            ApiResponse::new(
                Body::Err(format!("{{\"error\": \"Bad login\"}}")),
                Status::Unauthorized,
            )
        }
        (Err(e), _) => {
            error!("Can't get resident: {}", e);
            ApiResponse::new(
                Body::Err(format!(
                    "{{\"error\": \"Something terrible happened: {}\"}}",
                    e
                )),
                Status::ImATeapot,
            )
        }
        (_, None) => {
            error!("Can't compute token expiration");
            ApiResponse::new(
                Body::Err(format!(
                    "{{\"error\": \"Something terrible happened: Can't compute token expiration\"}}")),
                Status::ImATeapot,
            )
        }
    }
}
