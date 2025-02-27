use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUtilsDeliveryDto {
    pub delivery: Option<Uuid>,
    pub user: Option<Uuid>,
    pub date: Option<DateTimeUtc>,
    pub observations: Option<String>,
}
