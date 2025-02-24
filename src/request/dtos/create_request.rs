use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequestDto {
    pub justification: String,
    pub date: DateTimeUtc,
    pub status: String,
    pub user: Uuid,
    pub classroom: Uuid,
}
