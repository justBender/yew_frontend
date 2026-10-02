use gloo_net::http::{Request, Response};
use std::error::Error;
use web_sys::RequestCredentials;

#[derive(PartialEq, Clone)]
pub struct ResetRequest {
    pub password: String,
    pub token: String,
}

impl ResetRequest {
    pub async fn reset(&self) -> Result<Response, Box<dyn Error>> {
        let body = serde_json::json!({
            "password": self.password,
            "token": self.token
        });

        Ok(Request::post("http://localhost:7878/auth/reset-password")
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(&body)
            .unwrap()
            .send()
            .await
            .map_err(|e| e.to_string())?)
    }
}
