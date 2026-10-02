use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
}

impl SignupRequest {
    pub async fn signup(&self) -> Result<Response, Box<dyn Error>> {
        let body = serde_json::json!({
            "email": self.email,
            "name": self.name,
            "password": self.password,
            "roles": self.roles,
        });

        Ok(Request::post("http://localhost:7878/auth/register")
            .header("Content-Type", "application/json")
            .json(&body)
            .unwrap()
            .send()
            .await
            .map_err(|e| e.to_string())?)
    }
}
