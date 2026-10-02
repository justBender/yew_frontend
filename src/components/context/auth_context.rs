use crate::components::context::auth_state::AuthState;
use crate::services::login::LoginRequest;
use crate::services::logout::logout;
use crate::services::refresh::refresh;
use crate::services::reset::ResetRequest;
use crate::services::save_image::SaveImageRequest;
use crate::services::send_reset::SendResetRequest;
use crate::services::signup::SignupRequest;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AuthContext {
    pub state: AuthState,
}

impl AuthContext {
    pub async fn refresh() -> Result<AuthContext, String> {
        match refresh().await {
            Ok(res) => {
                let body: AuthState = res.json().await.map_err(|e| e.to_string())?;
                Ok(Self {
                    state: AuthState {
                        id: Some(body.id.unwrap_or_default()),
                        email: Some(body.email.unwrap_or_default()),
                        name: Some(body.name.unwrap_or_default()),
                        roles: Some(body.roles.unwrap_or_default()),
                        image: Some(body.image.unwrap_or_default()),
                        logged_in: true,
                    },
                })
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn login(login_request: LoginRequest) -> Result<AuthContext, String> {
        match login_request.login().await {
            Ok(response) => {
                let body: AuthState = response.json().await.map_err(|e| e.to_string())?;
                Ok(Self {
                    state: AuthState {
                        id: Some(body.id.unwrap_or_default()),
                        email: Some(body.email.unwrap_or_default()),
                        name: Some(body.name.unwrap_or_default()),
                        roles: Some(body.roles.unwrap_or_default()),
                        image: Some(body.image.unwrap_or_default()),
                        logged_in: true,
                    },
                })
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn logout() -> Result<AuthContext, String> {
        let result = logout().await;
        match result {
            Ok(res) => {
                let body: AuthState = res.json().await.map_err(|e| e.to_string())?;
                println!("{}", body.name.expect("missing name"));
                Ok(Self {
                    state: AuthState {
                        id: None,
                        email: None,
                        name: None,
                        roles: None,
                        image: None,
                        logged_in: false,
                    },
                })
            }
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn signup(signup_request: SignupRequest) -> Result<String, String> {
        match signup_request.signup().await {
            Ok(response) => Ok(response.json::<String>().await.map_err(|e| e.to_string())?),
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn reset(reset_request: ResetRequest) -> Result<String, String> {
        match reset_request.reset().await {
            Ok(response) => Ok(response.text().await.map_err(|e| e.to_string())?),
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn send_reset(reset_request: SendResetRequest) -> Result<String, String> {
        match reset_request.send_reset().await {
            Ok(response) => Ok(response.text().await.map_err(|e| e.to_string())?),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn save_image(save_image_request: SaveImageRequest) -> Result<AuthState, String> {
        match save_image_request.save_image().await {
            Ok(response) => Ok(response
                .json::<AuthState>()
                .await
                .map_err(|e| e.to_string())?),
            Err(err) => Err(err.to_string()),
        }
    }
}
