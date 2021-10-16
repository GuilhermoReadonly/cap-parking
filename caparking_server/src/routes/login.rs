use caparking_lib::{LoginForm, Token};
use rocket::{http::Status, serde::json::Json};

use crate::routes::{ApiResponse, Body};

#[post("/login", data = "<login_form>")]
pub fn login(login_form: Json<LoginForm>) -> ApiResponse<Token> {
    info!("login...");

    if login_form.login == login_form.password {
        ApiResponse::new(
            Body::Ok(Json(Token {
                token: format!("718718718"),
            })),
            Status::Ok,
        )
    } else {
        ApiResponse::new(
            Body::Err(format!("{{\"error\": \"Bad login\"}}")),
            Status::Unauthorized,
        )
    }
}
