use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AddUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct AddUserResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthenticateRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthenticateResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
}

impl AuthenticateResponse {
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
            token: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetPageRequest {
    pub token: String,
    pub slug: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetPageResponse {
    pub success: bool,
    pub message: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub version: Option<i32>,
}

impl GetPageResponse {
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
            title: None,
            body: None,
            version: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SetPageRequest {
    pub token: String,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub previous_version: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SetPageResponse {
    pub success: bool,
    pub message: String,
    pub new_version: Option<i32>,
}

impl SetPageResponse {
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message,
            new_version: None,
        }
    }
}
