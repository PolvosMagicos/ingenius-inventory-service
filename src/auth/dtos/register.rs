use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterUserDto {
    #[validate(length(min = 2, max = 50))]
    pub name: String,
    #[validate(length(min = 2, max = 50))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}
