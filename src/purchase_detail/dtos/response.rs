use serde::Serialize;
use uuid::Uuid;

use crate::purchase::dtos::PurchaseWithUser;

#[derive(Debug, Serialize)]
pub struct PurchaseDetailResponse {
    #[serde(flatten)]
    pub id: Uuid,
    pub util: entity::util::Model,
    pub purchase: PurchaseWithUser,
    pub unit_price: f64,
    pub amount: i32,
}
