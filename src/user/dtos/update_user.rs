use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub photo_url: Option<String>,
    pub role: Option<String>,
}
