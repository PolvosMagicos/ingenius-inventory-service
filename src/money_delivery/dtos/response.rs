use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct MoneyDeliveryWithUser {
    #[serde(flatten)]
    pub money_delivery: MoneyDeliveryResponse,
    pub user: Option<entity::user::Model>,
}

#[derive(Debug, Serialize)]
pub struct MoneyDeliveryResponse {
    #[serde(flatten)]
    pub id: Uuid,
    pub delivery_id: Uuid,
    pub amount: f64,
    pub date: String,
    pub observations: String,
}
