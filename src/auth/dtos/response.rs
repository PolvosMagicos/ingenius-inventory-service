use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub photo_url: String,
    pub role: String,
}
