use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub photo_url: String,
    pub role: String,
}
