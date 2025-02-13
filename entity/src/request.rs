use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "request")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub justification: String,
    pub date: DateTime,
    pub status: String,
    pub classroom_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::classroom::Entity",
        from = "Column::ClassroomId",
        to = "super::classroom::Column::Id"
    )]
    Classroom,
}

impl Related<super::classroom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classroom.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
