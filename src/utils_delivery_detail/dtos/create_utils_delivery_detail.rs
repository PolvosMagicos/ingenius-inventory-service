use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUtilsDeliveryDetailDto {
    pub utils_delivery_id: Uuid,
    pub util_id: Uuid,
    pub quantity: i32,
    pub state: String,
}
