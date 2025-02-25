use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct SchoolSupplyBalanceResponse {
    pub id: Uuid,
    pub balance: f64,
}