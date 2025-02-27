use serde::Serialize;
use uuid::Uuid;

use crate::user::dtos::response::UserResponse;

#[derive(Debug, Serialize)]
pub struct ClassroomWithUtilsList {
    #[serde(flatten)]
    pub classroom: ClassroomResponse,
    pub utils_list: Option<entity::utils_list::Model>,
    pub user: Option<UserResponse>,
}

#[derive(Debug, Serialize)]
pub struct ClassroomResponse {
    pub id: Uuid,
    pub name: String,
}
