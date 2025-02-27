use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub photo_url: String,
    pub role: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::request::Entity")]
    Request,
    #[sea_orm(has_one = "super::purchase::Entity")]
    Purchase,
    #[sea_orm(has_one = "super::utils_delivery::Entity")]
    UtilsDelivery,
    #[sea_orm(has_one = "super::money_delivery::Entity")]
    MoneyDelivery,
    #[sea_orm(has_many = "super::classroom::Entity")]
    Classroom,
}

impl Related<super::request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Request.def()
    }
}

impl Related<super::purchase::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Purchase.def()
    }
}

impl Related<super::utils_delivery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UtilsDelivery.def()
    }
}

impl Related<super::money_delivery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MoneyDelivery.def()
    }
}

impl Related<super::classroom::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classroom.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
