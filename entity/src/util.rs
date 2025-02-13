use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "util")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub stock: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::list_detail::Entity")]
    ListDetail,
    #[sea_orm(has_one = "super::request_detail::Entity")]
    RequestDetail,
}

impl Related<super::classroom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ListDetail.def()
    }
}

impl Related<super::request_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RequestDetail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
