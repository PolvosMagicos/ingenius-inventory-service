use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "request_detail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub util_id: i32,
    pub purchase_id: i32,
    pub unit_price: Decimal,
    pub quantity: i32,
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
