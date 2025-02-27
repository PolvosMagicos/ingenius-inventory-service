use sea_orm::prelude::Decimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreatePurchaseDetailDto {
    pub purchase_id: Uuid,
    pub util_id: Uuid,
    pub unit_price: Decimal,
    pub quantity: i32,
}
