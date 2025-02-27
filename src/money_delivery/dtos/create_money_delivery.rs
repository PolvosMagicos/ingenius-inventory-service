use sea_orm::prelude::{DateTimeUtc, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMoneyDeliveryDto {
    pub delivery_id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub date: DateTimeUtc,
    pub observations: Option<String>,
}
