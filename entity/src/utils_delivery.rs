use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "utils_delivery")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub date: DateTime,
    pub observations: String,
    pub delivery_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::delivery::Entity",
        from = "Column::DeliveryId",
        to = "super::delivery::Column::Id"
    )]
    Delivery,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_one = "super::utils_delivery_detail::Entity")]
    UtilsDeliveryDetail,
}

impl Related<super::delivery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Delivery.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::utils_delivery_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UtilsDeliveryDetail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
