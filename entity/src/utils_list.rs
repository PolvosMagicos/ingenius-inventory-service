use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "utils_list")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub total: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::classroom::Entity")]
    Classroom,
}

impl Related<super::classroom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classroom.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
