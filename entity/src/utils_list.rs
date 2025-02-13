use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "utils_list")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub total: f32,
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
