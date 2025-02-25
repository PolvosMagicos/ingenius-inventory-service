use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSchoolSupplyBalanceDto {
    pub balance: Option<f64>,
}
