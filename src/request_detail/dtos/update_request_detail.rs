use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequestDetailDto {
    pub request: Option<Uuid>,
    pub util: Option<Uuid>,
    pub quantity: Option<i32>,
}
