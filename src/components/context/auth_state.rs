use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct AuthState {
    pub id: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub roles: Option<Vec<String>>,
    pub image: Option<String>,
    pub logged_in: bool,
}
