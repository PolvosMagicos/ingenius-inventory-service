use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequestDetailDto {
    pub request: Uuid,
    pub util: Uuid,
    pub quantity: i32,
}
