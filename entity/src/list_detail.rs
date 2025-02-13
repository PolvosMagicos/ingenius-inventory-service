use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "list_detail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub utils_list_id: i32,
    pub util_id: i32,
    pub quantity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::utils_list::Entity",
        from = "Column::UtilsListId",
        to = "super::utils_list::Column::Id"
    )]
    UtilsList,
    #[sea_orm(
        belongs_to = "super::util::Entity",
        from = "Column::UtilId",
        to = "super::util::Column::Id"
    )]
    Util,
}

impl Related<super::classroom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UtilsList.def()
    }
}

impl Related<super::util::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Util.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
