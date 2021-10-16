use rocket::request::{FromRequest, Outcome};
use rocket::{http::Status, Request};

pub struct TokenDecoded<'r> {
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
impl<'r> FromRequest<'r> for TokenDecoded<'r> {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn check_token_presence(raw_token: Option<&str>) -> Result<RawToken, TokenError> {
            match raw_token {
                Some(t) => Ok(RawToken(t)),
                None => Err(TokenError::Missing),
            }
        }
        fn decode_raw_token(
            raw_token: Result<RawToken, TokenError>,
        ) -> Result<TokenDecoded, TokenError> {
            match raw_token {
                Ok(t) => {
                    if t.0.starts_with("718") {
                        Ok(TokenDecoded { raw_token: t.0 })
                    } else {
                        Err(TokenError::Invalid)
                    }
                }
                Err(e) => Err(e),
            }
        }
        fn validate_token(
            token: Result<TokenDecoded, TokenError>,
        ) -> Result<TokenDecoded, TokenError> {
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
            Err(token_error) => Outcome::Failure((Status::Unauthorized, token_error)),
        }
    }
}
