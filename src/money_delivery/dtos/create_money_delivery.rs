use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMoneyDeliveryDto {
    pub delivery_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub date: String,
    pub observations: Option<String>,
}
