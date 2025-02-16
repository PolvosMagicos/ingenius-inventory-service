use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "student")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub classroom_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::classroom::Entity",
        from = "Column::ClassroomId",
        to = "super::classroom::Column::Id"
    )]
    Classroom,
    #[sea_orm(has_one = "super::delivery::Entity")]
    Delivery,
}

impl Related<super::classroom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classroom.def()
    }
}

impl Related<super::delivery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Delivery.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
