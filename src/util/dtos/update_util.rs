use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUtilDto {
    pub name: Option<String>,
    pub stock: Option<i32>,
}
