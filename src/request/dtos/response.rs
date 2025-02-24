use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;

use crate::{classroom::dtos::ClassroomWithUtilsList, user::dtos::response::UserResponse};

#[derive(Debug, Serialize)]
pub struct RequestResponse {
    #[serde(flatten)]
    pub id: String,
    pub justification: String,
    pub date: DateTimeUtc,
    pub status: String,
    pub classroom: ClassroomWithUtilsList,
    pub user: UserResponse,
}
