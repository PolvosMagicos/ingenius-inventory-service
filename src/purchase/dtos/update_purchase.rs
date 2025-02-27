use sea_orm::prelude::{DateTimeUtc, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePurchaseDto {
    pub total_spent: Option<Decimal>,
    pub date: Option<DateTimeUtc>,
    pub user_id: Option<Uuid>,
}
