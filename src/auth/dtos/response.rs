use serde::Serialize;

use crate::user::dtos::response;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: response::UserResponse,
}
