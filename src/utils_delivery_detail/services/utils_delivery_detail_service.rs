use crate::delivery::dtos::{DeliveryResponse, DeliveryWithStudent};
use crate::user::dtos::response::UserResponse;
use crate::utils_delivery::dtos::UtilsDeliveryWithDelivery;
use crate::utils_delivery_detail::dtos::{
    CreateUtilsDeliveryDetailDto, UpdateUtilsDeliveryDetailDto, UtilsDeliveryDetailResponse,
};
use entity::delivery;
use entity::{
    student, user, util, utils_delivery,
    utils_delivery_detail::{ActiveModel, Entity, Model},
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

pub struct UtilsDeliveryDetailService;

impl UtilsDeliveryDetailService {
    pub async fn get_utils_delivery_detail(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<UtilsDeliveryDetailResponse>, DbErr> {
        let result = Entity::find_by_id(id)
            .find_also_related(util::Entity)
            .one(db)
            .await?;

        if let Some((detail, util)) = result {
            let utils_delivery = utils_delivery::Entity::find_by_id(detail.utils_delivery_id)
                .one(db)
                .await?;

            let delivery =
                delivery::Entity::find_by_id(utils_delivery.clone().unwrap().delivery_id)
                    .one(db)
                    .await?
                    .unwrap();

            let student = student::Entity::find_by_id(delivery.student_id)
                .one(db)
                .await?
                .unwrap();

            if let Some(utils_delivery) = utils_delivery {
                let user = user::Entity::find_by_id(utils_delivery.user_id)
                    .one(db)
                    .await?
                    .unwrap();

                let user_response = UserResponse {
                    id: user.id,
                    name: user.name,
                    last_name: user.last_name,
                    email: user.email,
                    photo_url: user.photo_url,
                    role: user.role,
                };

                let delivery_response = DeliveryWithStudent {
                    delivery: DeliveryResponse {
                        id: delivery.id,
                        status: delivery.state,
                    },
                    student: Some(student),
                };

                let utils_delivery_with_delivery = UtilsDeliveryWithDelivery {
                    id,
                    delivery: delivery_response,
                    user: user_response,
                    date: utils_delivery.date.and_utc(),
                    observations: utils_delivery.observations,
                };

                return Ok(Some(UtilsDeliveryDetailResponse {
                    id: detail.id,
                    util: util.unwrap(),
                    utils_delivery: utils_delivery_with_delivery,
                    quantity: detail.quantity,
                    state: detail.state,
                }));
            }
        }

        Ok(None)
    }

    pub async fn get_all_utils_delivery_details(
        db: &DatabaseConnection,
    ) -> Result<Vec<UtilsDeliveryDetailResponse>, DbErr> {
        // Get all utils_delivery_details with related util
        let results = Entity::find()
            .find_also_related(util::Entity)
            .all(db)
            .await?;

        let mut details_responses = Vec::new();

        for (detail, util) in results {
            let utils_delivery = utils_delivery::Entity::find_by_id(detail.utils_delivery_id)
                .find_also_related(delivery::Entity)
                .one(db)
                .await?;

            if let Some((utils_delivery, delivery)) = utils_delivery {
                let user = user::Entity::find_by_id(utils_delivery.user_id)
                    .one(db)
                    .await?
                    .unwrap();

                let student = student::Entity::find_by_id(delivery.clone().unwrap().student_id)
                    .one(db)
                    .await?
                    .unwrap();

                let user_response = UserResponse {
                    id: user.id,
                    name: user.name,
                    last_name: user.last_name,
                    email: user.email,
                    photo_url: user.photo_url,
                    role: user.role,
                };

                let delivery_response = DeliveryWithStudent {
                    delivery: DeliveryResponse {
                        id: delivery.clone().unwrap().id,
                        status: delivery.clone().unwrap().state,
                    },
                    student: Some(student),
                };

                let utils_delivery_response = UtilsDeliveryWithDelivery {
                    id: utils_delivery.id,
                    delivery: delivery_response,
                    user: user_response,
                    date: utils_delivery.date.and_utc(),
                    observations: utils_delivery.observations,
                };

                details_responses.push(UtilsDeliveryDetailResponse {
                    id: detail.id,
                    util: util.clone().unwrap(),
                    utils_delivery: utils_delivery_response,
                    quantity: detail.quantity,
                    state: detail.clone().state,
                });
            }
        }

        Ok(details_responses)
    }

    pub async fn create_utils_delivery_detail(
        db: &DatabaseConnection,
        detail_dto: CreateUtilsDeliveryDetailDto,
    ) -> Result<Model, DbErr> {
        let detail = ActiveModel {
            id: Set(Uuid::new_v4()),
            utils_delivery_id: Set(detail_dto.utils_delivery_id),
            util_id: Set(detail_dto.util_id),
            quantity: Set(detail_dto.quantity),
            state: Set(detail_dto.state),
        };

        detail.insert(db).await
    }

    pub async fn update_utils_delivery_detail(
        db: &DatabaseConnection,
        id: Uuid,
        detail_dto: UpdateUtilsDeliveryDetailDto,
    ) -> Result<Model, DbErr> {
        let detail = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_detail) = detail {
            let mut detail_active_model: ActiveModel = existing_detail.into();

            if let Some(utils_delivery_id) = detail_dto.utils_delivery_id {
                detail_active_model.utils_delivery_id = Set(utils_delivery_id);
            }

            if let Some(util_id) = detail_dto.util_id {
                detail_active_model.util_id = Set(util_id);
            }

            if let Some(quantity) = detail_dto.quantity {
                detail_active_model.quantity = Set(quantity);
            }

            if let Some(state) = detail_dto.state {
                detail_active_model.state = Set(state);
            }

            detail_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound(
                "UtilsDeliveryDetail not found".to_string(),
            ))
        }
    }

    pub async fn delete_utils_delivery_detail(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
