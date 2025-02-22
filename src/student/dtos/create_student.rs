use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStudentDto {
    pub name: String,
    pub last_name: String,
    pub classroom_id: Uuid,
}
