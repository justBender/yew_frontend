use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use web_sys::RequestCredentials;

#[derive(PartialEq, Clone, Debug, Default, Deserialize, Serialize)]
pub struct SendResetRequest {
    pub email: String,
}

impl SendResetRequest {
    pub async fn send_reset(&self) -> Result<Response, Box<dyn Error>> {
        let body = serde_json::json!({
            "email": self.email
        });
        Ok(
            Request::post("http://localhost:7878/auth/password-reset-request")
                .header("Content-Type", "application/json")
                .credentials(RequestCredentials::Include)
                .json(&body)
                .unwrap()
                .send()
                .await
                .map_err(|e| e.to_string())?,
        )
    }
}
