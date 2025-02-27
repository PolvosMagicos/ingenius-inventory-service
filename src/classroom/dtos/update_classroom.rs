use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClassroomDto {
    pub name: Option<String>,
    pub utils_list: Option<Uuid>,
    pub user: Option<Uuid>,
}
