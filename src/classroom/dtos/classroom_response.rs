use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ClassroomWithUtilsList {
    #[serde(flatten)]
    pub classroom: ClassroomResponse,
    pub utils_list: Option<entity::utils_list::Model>,
}

#[derive(Debug, Serialize)]
pub struct ClassroomResponse {
    pub id: Uuid,
    pub name: String,
}
