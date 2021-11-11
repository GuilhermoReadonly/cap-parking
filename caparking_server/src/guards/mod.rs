use caparking_lib::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::log::private::warn;
use rocket::request::{FromRequest, Outcome};
use rocket::State;
use rocket::{http::Status, Request};

use crate::Secret;

pub struct SecurityGuard {
    pub decoded_token: Claims,
}
struct RawToken<'r>(&'r str);

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SecurityGuard {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn check_token_presence(raw_token: Option<&str>) -> Result<RawToken<'_>, TokenError> {
            match raw_token {
                Some(t) => Ok(RawToken(t)),
                None => Err(TokenError::Missing),
            }
        }
        async fn decode_raw_token(
            raw_token: Result<RawToken<'_>, TokenError>,
            req: &'_ Request<'_>,
        ) -> Result<SecurityGuard, TokenError> {
            match raw_token {
                Ok(t) => {
                    let secret = req
                        .guard::<&State<Secret>>()
                        .await
                        .expect("Secret not present !")
                        .0
                        .clone();
                    let decoded_token = decode::<Claims>(
                        &t.0,
                        &DecodingKey::from_secret(secret.as_ref()),
                        &Validation::default(),
                    );

                    match decoded_token {
                        Ok(t) => Ok(SecurityGuard {
                            decoded_token: t.claims,
                        }),
                        Err(e) => {
                            warn!("Can't decode token: {}", e);
                            Err(TokenError::Invalid)
                        }
                    }
                }
                Err(e) => Err(e),
            }
        }

        let option_raw_token = req.headers().get_one("Authorization");
        let raw_token = check_token_presence(option_raw_token);
        let token_decoded = decode_raw_token(raw_token, req).await;

        match token_decoded {
            Ok(t) => Outcome::Success(t),
            Err(token_error) => Outcome::Failure((Status::Unauthorized, token_error)),
        }
    }
}
