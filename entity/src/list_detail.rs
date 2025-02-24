use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "list_detail")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub quantity: i32,
    pub utils_list_id: Uuid,
    pub util_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::util::Entity",
        from = "Column::UtilId",
        to = "super::util::Column::Id"
    )]
    Util,
    #[sea_orm(
        belongs_to = "super::utils_list::Entity",
        from = "Column::UtilsListId",
        to = "super::utils_list::Column::Id"
    )]
    UtilsList,
}

impl Related<super::utils_list::Entity> for Entity {
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
