use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "classroom")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(nullable)]
    pub utils_list_id: Option<Uuid>,
    #[sea_orm(nullable)]
    pub user_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::student::Entity")]
    Student,
    #[sea_orm(has_many = "super::request::Entity")]
    Request,
    #[sea_orm(
        belongs_to = "super::utils_list::Entity",
        from = "Column::UtilsListId",
        to = "super::utils_list::Column::Id"
    )]
    UtilsList,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl Related<super::utils_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UtilsList.def()
    }
}

impl Related<super::request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Request.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
