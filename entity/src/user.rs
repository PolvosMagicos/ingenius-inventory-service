use sea_orm::{entity::prelude::*, sqlx::types::Text};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub last_name: String,
    pub email:String ,
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

impl ActiveModelBehavior for ActiveModel {}
