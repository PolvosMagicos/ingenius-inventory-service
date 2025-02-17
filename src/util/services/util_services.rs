use entity::util::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::util::dtos::{CreateUtilDto, UpdateUtilDto};

pub struct UtilService;

impl UtilService {
    pub async fn get_util(db: &DatabaseConnection, id: Uuid) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn get_all_utils(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }

    pub async fn create_util(
        db: &DatabaseConnection,
        util_dto: CreateUtilDto,
    ) -> Result<Model, DbErr> {
        let util = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(util_dto.name),
            stock: Set(util_dto.stock),
        };

        util.insert(db).await
    }

    pub async fn update_util(
        db: &DatabaseConnection,
        id: Uuid,
        util_dto: UpdateUtilDto,
    ) -> Result<Model, DbErr> {
        let util = Entity::find_by_id(id).one(db).await?;

        if let Some(util) = util {
            let mut util: ActiveModel = util.into();
            if let Some(name) = util_dto.name {
                util.name = Set(name);
            }
            if let Some(stock) = util_dto.stock {
                util.stock = Set(stock);
            }

            util.update(db).await
        } else {
            Err(DbErr::RecordNotFound("Util not found".to_string()))
        }
    }

    pub async fn delete_util(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
