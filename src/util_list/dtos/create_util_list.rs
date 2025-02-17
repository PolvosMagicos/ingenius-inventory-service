use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUtilListDto {
    pub name: String,
    pub total: Decimal,
}
