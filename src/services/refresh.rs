use gloo_net::http::{Request, Response};
use std::error::Error;
use web_sys::RequestCredentials;

pub async fn refresh() -> Result<Response, Box<dyn Error>> {
    Ok(Request::post("http://localhost:7878/auth/refresh")
        .credentials(RequestCredentials::Include)
        .build()
        .unwrap()
        .send()
        .await
        .map_err(|e| e.to_string())?)
}
