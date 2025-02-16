use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClassroomDto {
    pub name: String,
    pub utils_list_id: Option<i32>,
}
