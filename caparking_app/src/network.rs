use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchError {
    err: Option<String>,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self {
            err: value.as_string(),
        }
    }
}

pub async fn request<A: Serialize, B: for<'a> Deserialize<'a>>(
    verb: &str,
    url: &str,
    body: Option<A>,
) -> Result<B, FetchError> {
    let mut opts = RequestInit::new();

    let js_value = serde_json::json!(body);
    let js_value = JsValue::from_str(&js_value.to_string());

    opts.method(&verb);
    //opts.headers(&"{\"Authorization\": \"718718123456\"}".into());
    if body.is_some() {
        opts.body(Some(&js_value));
    }

    let request = Request::new_with_str_and_init(url, &opts)?;
    request.headers().set("Authorization", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2MzYyNDgyMzYsInN1YiI6eyJpZCI6NywibmFtZSI6IkxvcyBSYWRvbmRpw7FvcyIsImxvZ2luIjoiNzE4IiwicGFya2luZ19zcG90cyI6WzFdfX0.r1lH6Y_t_qDaVYm0dJYTU_ykLYfru0XpfoyXjx61Xdc")?;

    let window = yew::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let js_value = JsFuture::from(resp.json()?).await?;
    let data = js_value.into_serde().map_err(|e| FetchError {
        err: Some(format!("Can't parse response: {:?}", e)),
    })?;
    Ok(data)
}
