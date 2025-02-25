use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeliveryDto {
    pub status: String,
    pub student_id: Option<Uuid>,
}
