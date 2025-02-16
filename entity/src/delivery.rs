use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "delivery")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub state: String,
    pub student_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::StudentId",
        to = "super::student::Column::Id"
    )]
    Student,
    #[sea_orm(has_many = "super::utils_delivery::Entity")]
    UtilsDelivery,
    #[sea_orm(has_many = "super::money_delivery::Entity")]
    MoneyDelivery,
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
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

impl ActiveModelBehavior for ActiveModel {}
