use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUtilListDto {
    pub name: Option<String>,
    pub total: Option<Decimal>,
}
