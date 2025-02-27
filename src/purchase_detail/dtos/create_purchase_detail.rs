use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreatePurchaseDetailDto {
    pub purchase_id: Uuid,
    pub util_id: Uuid,
    pub unit_price: f64,
    pub amount: i32,
}
