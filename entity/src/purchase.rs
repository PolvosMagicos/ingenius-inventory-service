use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "request")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub total_spent: Decimal,
    pub date: DateTime,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_one = "super::purchase_detail::Entity")]
    PurchaseDetail,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::purchase_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PurchaseDetail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
