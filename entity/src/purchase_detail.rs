use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "purchase_detail")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub unit_price: Decimal,
    pub quantity: i32,
    pub util_id: Uuid,
    pub purchase_id: Uuid,
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
        belongs_to = "super::purchase::Entity",
        from = "Column::PurchaseId",
        to = "super::purchase::Column::Id"
    )]
    Purchase,
}

impl Related<super::util::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Util.def()
    }
}
impl Related<super::purchase::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Purchase.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
