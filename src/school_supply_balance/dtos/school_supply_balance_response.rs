use sea_orm::prelude::Decimal;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct SchoolSupplyBalanceResponse {
    pub id: Uuid,
    pub balance: Decimal,
}
