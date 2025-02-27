use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePurchaseDto {
    pub total_spent: Option<f64>,
    pub date: Option<String>,
    pub user_id: Option<Uuid>,
}
