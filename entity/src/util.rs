use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "util")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub stock: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::list_detail::Entity")]
    ListDetail,
    #[sea_orm(has_one = "super::request_detail::Entity")]
    RequestDetail,
    #[sea_orm(has_one = "super::purchase_detail::Entity")]
    PurchaseDetail,
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

impl Related<super::purchase_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PurchaseDetail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
