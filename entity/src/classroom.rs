use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "classroom")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub utils_list_id: Uuid,
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
impl ActiveModelBehavior for ActiveModel {}
