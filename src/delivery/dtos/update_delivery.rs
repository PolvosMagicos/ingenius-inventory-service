use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeliveryDto {
    pub status: Option<String>,
    pub student_id: Option<Uuid>,
}
