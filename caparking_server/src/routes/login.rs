use caparking_lib::{LoginForm, Token};
use rocket::{http::Status, serde::json::Json};

use crate::routes::{ApiResponse, Body};

#[post("/login", data = "<login_form>")]
pub fn login(login_form: Json<LoginForm>) -> ApiResponse<Token> {
    info!("login...");

    let resident = caparking_lib::get_resident_by_login(login_form.login.clone());
    match resident {
        Ok(Some(r)) => {
            if login_form.password == r.password {
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
        Ok(None) => ApiResponse::new(
            Body::Err(format!("{{\"error\": \"Bad login\"}}")),
            Status::Unauthorized,
        ),
        Err(e) => ApiResponse::new(
            Body::Err(format!(
                "{{\"error\": \"Something terrible happened: {}\"}}",
                e
            )),
            Status::ImATeapot,
        ),
    }
}
