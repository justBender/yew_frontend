use gloo_net::http::{Request, Response};
use std::error::Error;
use web_sys::RequestCredentials;

#[derive(PartialEq, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl LoginRequest {
    pub async fn login(&self) -> Result<Response, Box<dyn Error>> {
        let body = serde_json::json!({
            "email": self.email,
            "password": self.password
        });

        Ok(Request::post("http://localhost:7878/auth/login")
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(&body)
            .unwrap()
            .send()
            .await
            .map_err(|e| e.to_string())?)
    }
}
