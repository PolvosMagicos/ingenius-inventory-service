use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequestDto {
    pub justification: Option<String>,
    pub date: Option<DateTimeUtc>,
    pub status: Option<String>,
    pub user: Option<Uuid>,
    pub classroom: Option<Uuid>,
}
