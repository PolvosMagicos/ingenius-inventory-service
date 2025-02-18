use entity::utils_list::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::util_list::dtos::{
    create_util_list::CreateUtilListDto, update_util_list::UpdateUtilListDto,
};

pub struct UtilListService;

impl UtilListService {
    pub async fn get_util_list(db: &DatabaseConnection, id: Uuid) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn get_all_util_lists(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }

    pub async fn create_util_list(
        db: &DatabaseConnection,
        util_list_dto: CreateUtilListDto,
    ) -> Result<Model, DbErr> {
        let util_list = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(util_list_dto.name),
            total: Set(util_list_dto.total),
        };

        util_list.insert(db).await
    }

    pub async fn update_util_list(
        db: &DatabaseConnection,
        id: Uuid,
        util_list_dto: UpdateUtilListDto,
    ) -> Result<Model, DbErr> {
        let util_list = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_util_list) = util_list {
            let mut util_list_active_model: ActiveModel = existing_util_list.into();

            if let Some(name) = util_list_dto.name {
                util_list_active_model.name = Set(name);
            }
            if let Some(total) = util_list_dto.total {
                util_list_active_model.total = Set(total);
            }

            util_list_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound("Util list not found".to_string()))
        }
    }

    pub async fn delete_util_list(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
