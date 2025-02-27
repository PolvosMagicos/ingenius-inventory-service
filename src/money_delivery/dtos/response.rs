use sea_orm::prelude::{DateTimeUtc, Decimal};
use serde::Serialize;
use uuid::Uuid;

use crate::user::dtos::response::UserResponse;

#[derive(Debug, Serialize)]
pub struct MoneyDeliveryWithUser {
    #[serde(flatten)]
    pub money_delivery: MoneyDeliveryResponse,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct MoneyDeliveryResponse {
    #[serde(flatten)]
    pub id: Uuid,
    pub delivery_id: Uuid,
    pub amount: Decimal,
    pub date: DateTimeUtc,
    pub observations: String,
}
