use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSchoolSupplyBalanceDto {
    pub balance: Decimal,
}
