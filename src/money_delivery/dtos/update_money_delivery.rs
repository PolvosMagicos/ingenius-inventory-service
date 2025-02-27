use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMoneyDeliveryDto {
    pub delivery_id: Option<Uuid>,
    pub amount: Option<f64>,
    pub date: Option<String>,
    pub observations: Option<String>,
}
