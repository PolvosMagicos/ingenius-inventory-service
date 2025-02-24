use entity::{
    request_detail::{ActiveModel, Entity, Model},
    util,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::request_detail::dtos::{
    create_request_detail::CreateRequestDetailDto,
    response::{RequestDetailResponse, RequestDetailWithUtil},
    update_request_detail::UpdateRequestDetailDto,
};

pub struct RequestDetailService;

impl RequestDetailService {
    pub async fn get_request_detail(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<RequestDetailWithUtil>, DbErr> {
        let result = Entity::find_by_id(id)
            .find_also_related(util::Entity)
            .one(db)
            .await?;
        Ok(result.map(|(request_detail, util)| {
            let request_response = RequestDetailResponse {
                id: request_detail.id,
                request: request_detail.request_id,
                quantity: request_detail.quantity,
            };
            RequestDetailWithUtil {
                request_detail: request_response,
                util,
            }
        }))
    }

    pub async fn get_all_resquests_details(
        db: &DatabaseConnection,
    ) -> Result<Vec<RequestDetailWithUtil>, DbErr> {
        let results = Entity::find()
            .find_also_related(util::Entity)
            .all(db)
            .await?;

        let classrooms_with_utils = results
            .into_iter()
            .map(|(request_detail, util)| {
                let request_response = RequestDetailResponse {
                    id: request_detail.id,
                    request: request_detail.request_id,
                    quantity: request_detail.quantity,
                };
                RequestDetailWithUtil {
                    request_detail: request_response,
                    util,
                }
            })
            .collect();
        Ok(classrooms_with_utils)
    }

    pub async fn create_request_detail(
        db: &DatabaseConnection,
        request_detail_dto: CreateRequestDetailDto,
    ) -> Result<Model, DbErr> {
        let request_detail = ActiveModel {
            id: Set(Uuid::new_v4()),
            request_id: Set(request_detail_dto.request),
            util_id: Set(request_detail_dto.util),
            quantity: Set(request_detail_dto.quantity),
        };

        request_detail.insert(db).await
    }

    pub async fn update_request_detail(
        db: &DatabaseConnection,
        id: Uuid,
        request_detail_dto: UpdateRequestDetailDto,
    ) -> Result<Model, DbErr> {
        let request_detail = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_request_detail) = request_detail {
            let mut request_detail_active_model: ActiveModel = existing_request_detail.into();

            if let Some(request) = request_detail_dto.request {
                request_detail_active_model.request_id = Set(request);
            }

            if let Some(util) = request_detail_dto.util {
                request_detail_active_model.util_id = Set(util);
            }

            if let Some(quantity) = request_detail_dto.quantity {
                request_detail_active_model.quantity = Set(quantity);
            }

            request_detail_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound(
                "Request detail not found".to_string(),
            ))
        }
    }

    pub async fn delete_request_detail(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
