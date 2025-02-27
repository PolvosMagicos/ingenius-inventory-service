use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUtilsDeliveryDto {
    pub delivery: Uuid,
    pub user: Uuid,
    pub date: DateTimeUtc,
    pub observations: String,
}
