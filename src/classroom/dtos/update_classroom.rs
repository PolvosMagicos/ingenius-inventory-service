use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UpdateClassroomDto {
    pub name: Option<String>,
    pub utils_list_id: Option<Uuid>,
}
