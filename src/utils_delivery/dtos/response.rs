use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    delivery::dtos::delivery_response::DeliveryWithStudent, user::dtos::response::UserResponse,
};

#[derive(Debug, Serialize)]
pub struct UtilsDeliveryWithDelivery {
    #[serde(flatten)]
    pub id: Uuid,
    pub delivery: DeliveryWithStudent,
    pub user: UserResponse,
    pub date: DateTimeUtc,
    pub observations: String,
}
