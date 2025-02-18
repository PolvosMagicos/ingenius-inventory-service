use entity::list_detail::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::list_detail::dtos::{
    create_list_detail::CreateListDetailDto, update_list_detail::UpdateListDetailDto,
};

pub struct ListDetailService;

impl ListDetailService {
    pub async fn get_list_detail(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn get_all_lists_details(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }

    pub async fn create_list_detail(
        db: &DatabaseConnection,
        list_detail_dto: CreateListDetailDto,
    ) -> Result<Model, DbErr> {
        let list_detail = ActiveModel {
            id: Set(Uuid::new_v4()),
            utils_list_id: Set(list_detail_dto.utils_list),
            util_id: Set(list_detail_dto.util),
            quantity: Set(list_detail_dto.quantity),
        };

        list_detail.insert(db).await
    }

    pub async fn update_list_detail(
        db: &DatabaseConnection,
        id: Uuid,
        update_list_detail_dto: UpdateListDetailDto,
    ) -> Result<Model, DbErr> {
        let list_detail = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_list_detail) = list_detail {
            let mut list_detail_active_model: ActiveModel = existing_list_detail.into();

            if let Some(utils_list_id) = update_list_detail_dto.utils_list {
                list_detail_active_model.utils_list_id = Set(utils_list_id);
            }

            if let Some(util_id) = update_list_detail_dto.util {
                list_detail_active_model.util_id = Set(util_id);
            }

            if let Some(quantity) = update_list_detail_dto.quantity {
                list_detail_active_model.quantity = Set(quantity);
            }

            list_detail_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound("List detail not found".to_string()))
        }
    }

    pub async fn delete_list_detail(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
