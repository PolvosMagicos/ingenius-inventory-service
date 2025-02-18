use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClassroomDto {
    pub name: String,
    pub utils_list: Option<Uuid>,
}
