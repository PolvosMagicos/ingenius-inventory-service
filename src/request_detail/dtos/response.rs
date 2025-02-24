use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RequestDetailWithUtil {
    #[serde(flatten)]
    pub request_detail: RequestDetailResponse,
    pub util: Option<entity::util::Model>,
}

#[derive(Debug, Serialize)]
pub struct RequestDetailResponse {
    #[serde(flatten)]
    pub id: Uuid,
    pub request: Uuid,
    pub quantity: i32,
}
