use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "utils_delivery_detail")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub quantity: i32,
    pub state: String,
    pub util_id: Uuid,
    pub utils_delivery_id: Uuid,
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
        belongs_to = "super::utils_delivery::Entity",
        from = "Column::UtilsDeliveryId",
        to = "super::utils_delivery::Column::Id"
    )]
    UtilsDelivery,
}

impl Related<super::util::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Util.def()
    }
}

impl Related<super::utils_delivery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UtilsDelivery.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
