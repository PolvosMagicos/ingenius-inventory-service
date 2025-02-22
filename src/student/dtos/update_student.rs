use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStudentDto {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub classroom_id: Option<Uuid>,
}
