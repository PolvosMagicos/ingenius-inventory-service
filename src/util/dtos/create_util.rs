use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUtilDto {
    pub name: String,
    pub stock: i32,
}
