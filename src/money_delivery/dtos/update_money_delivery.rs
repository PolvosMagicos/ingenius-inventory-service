use sea_orm::prelude::{DateTimeUtc, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMoneyDeliveryDto {
    pub user_id: Option<Uuid>,
    pub delivery_id: Option<Uuid>,
    pub amount: Option<Decimal>,
    pub date: Option<DateTimeUtc>,
    pub observations: Option<String>,
}
