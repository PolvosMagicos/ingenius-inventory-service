use sea_orm::prelude::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::purchase::dtos::PurchaseResponse;

#[derive(Debug, Serialize)]
pub struct PurchaseDetailResponse {
    pub id: Uuid,
    pub util: entity::util::Model,
    pub purchase: PurchaseResponse,
    pub unit_price: Decimal,
    pub quantity: i32,
}
