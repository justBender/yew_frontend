use gloo_net::http::Request;

pub async fn login(
    email: String,
    password: String,
) -> Result<String, String> {

    let body = serde_json::json!({
        "email": email,
        "password": password
    });

    let response =
        Request::post("/api/auth/login")
            .json(&body)
            .unwrap()
            .send()
            .await
            .map_err(|e| e.to_string())?;

    if response.ok() {
        response.text().await.map_err(|e| e.to_string())
    } else {
        Err(response.text().await.unwrap_or_default())
    }
}