use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct DeliveryWithStudent {
    #[serde(flatten)]
    pub delivery: DeliveryResponse,
    pub student: Option<entity::student::Model>, 
}

#[derive(Debug, Serialize)]
pub struct DeliveryResponse {
    pub id: Uuid,
    pub status: String,
}
