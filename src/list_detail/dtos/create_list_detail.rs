use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListDetailDto {
    pub utils_list: Uuid,
    pub util: Uuid,
    pub quantity: i32,
}
