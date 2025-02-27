use sea_orm::prelude::Decimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdatePurchaseDetailDto {
    pub purchase_id: Option<Uuid>,
    pub util_id: Option<Uuid>,
    pub unit_price: Option<Decimal>,
    pub quantity: Option<i32>,
}
