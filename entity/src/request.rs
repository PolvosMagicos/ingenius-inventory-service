use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "request")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub justification: String,
    pub date: DateTime,
    pub status: String,
    pub classroom_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::classroom::Entity",
        from = "Column::ClassroomId",
        to = "super::classroom::Column::Id"
    )]
    Classroom,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::request_detail::Entity")]
    RequestDetail,
}

impl Related<super::classroom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classroom.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::request_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RequestDetail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
