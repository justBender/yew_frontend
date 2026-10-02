use gloo_net::http::{Request, Response};
use std::error::Error;
use web_sys::RequestCredentials;

pub async fn logout() -> Result<Response, Box<dyn Error>> {
    let response = Request::get("http://localhost:7878/auth/logout")
        // .credentials() is necessary to send the HTTPOnly cookies
        .credentials(RequestCredentials::Include)
        .build()
        .unwrap()
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
}
