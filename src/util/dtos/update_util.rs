use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateUtilDto {
    pub name: Option<String>,
    pub stock: Option<i32>,
}
