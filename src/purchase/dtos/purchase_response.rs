use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseResponse {
    pub id: Uuid,
    pub total_spent: f64,
    pub date: String,
    pub user_id: Uuid,
}
