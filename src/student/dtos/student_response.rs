use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentResponse {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
}
