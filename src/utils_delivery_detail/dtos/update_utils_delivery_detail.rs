use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateUtilsDeliveryDetailDto {
    pub utils_delivery_id: Option<Uuid>,
    pub util_id: Option<Uuid>,
    pub quantity: Option<i32>,
    pub state: Option<String>,
}
