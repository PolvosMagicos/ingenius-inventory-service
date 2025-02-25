use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSchoolSupplyBalanceDto {
    pub balance: f64,
}
