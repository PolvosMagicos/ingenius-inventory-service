use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "classroom")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub utils_list_id: i32,
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
