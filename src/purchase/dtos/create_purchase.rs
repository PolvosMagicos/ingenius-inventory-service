use sea_orm::prelude::{DateTimeUtc, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePurchaseDto {
    pub total_spent: Decimal,
    pub date: DateTimeUtc,
    pub user_id: Uuid,
}
