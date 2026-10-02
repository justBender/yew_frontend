use gloo_net::http::{Request, Response};
use js_sys::Uint8Array;
use std::error::Error;
use web_sys::RequestCredentials;

#[derive(Clone)]
pub struct SaveImageRequest {
    pub user_id: String,
    pub bytes: Uint8Array,
    pub file_extension: String,
}

impl SaveImageRequest {
    pub async fn save_image(&self) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "{}/{}.{}",
            "http://localhost:7878/auth/save-image", self.user_id, self.file_extension
        );
        Ok(Request::post(&url)
            .header("Content-Type", "multipart/form-data")
            .credentials(RequestCredentials::Include)
            .body(self.bytes.clone())
            .unwrap()
            .send()
            .await
            .map_err(|e| e.to_string())?)
    }
}
