use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateClassroomDto {
    pub name: String,
    pub utils_list_id: Option<Uuid>,
}
