use serde::Serialize;
use uuid::Uuid;

use crate::utils_delivery::dtos::UtilsDeliveryWithDelivery;

#[derive(Debug, Serialize)]
pub struct UtilsDeliveryDetailResponse {
    #[serde(flatten)]
    pub id: Uuid,
    pub util: entity::util::Model,
    pub utils_delivery: UtilsDeliveryWithDelivery,
    pub quantity: i32,
    pub state: String,
}
