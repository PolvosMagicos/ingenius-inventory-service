use crate::{
    delivery::dtos::{delivery_response::DeliveryWithStudent, DeliveryResponse},
    user::dtos::response::UserResponse,
    utils_delivery::dtos::{
        CreateUtilsDeliveryDto, UpdateUtilsDeliveryDto, UtilsDeliveryWithDelivery,
    },
};
use entity::{
    delivery, student, user,
    utils_delivery::{ActiveModel, Entity, Model},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

pub struct UtilsDeliveryService;

impl UtilsDeliveryService {
    pub async fn get_utils_delivery(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<UtilsDeliveryWithDelivery>, DbErr> {
        let result = Entity::find_by_id(id)
            .find_also_related(delivery::Entity)
            .one(db)
            .await?;

        if let Some((utils_delivery, delivery)) = result {
            let delivery = delivery.unwrap();

            let student = student::Entity::find_by_id(delivery.student_id)
                .one(db)
                .await?
                .unwrap();

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

            return Ok(Some(UtilsDeliveryWithDelivery {
                id,
                delivery: delivery_response,
                user: user_response,
                date: utils_delivery.date.and_utc(),
                observations: utils_delivery.observations,
            }));
        }

        Ok(None)
    }

    pub async fn get_all_utils_deliveries(
        db: &DatabaseConnection,
    ) -> Result<Vec<UtilsDeliveryWithDelivery>, DbErr> {
        let utils_deliveries = Entity::find().all(db).await?;

        let mut result = Vec::new();

        for utils_delivery in utils_deliveries {
            let delivery = match delivery::Entity::find_by_id(utils_delivery.delivery_id)
                .one(db)
                .await?
            {
                Some(d) => d,
                None => continue,
            };

            let student = match student::Entity::find_by_id(delivery.student_id)
                .one(db)
                .await?
            {
                Some(s) => s,
                None => continue,
            };

            let user = match user::Entity::find_by_id(utils_delivery.user_id)
                .one(db)
                .await?
            {
                Some(u) => u,
                None => continue,
            };

            let delivery_response = DeliveryWithStudent {
                delivery: DeliveryResponse {
                    id: delivery.id,
                    status: delivery.state,
                },
                student: Some(student),
            };

            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                last_name: user.last_name,
                email: user.email,
                photo_url: user.photo_url,
                role: user.role,
            };

            result.push(UtilsDeliveryWithDelivery {
                id: utils_delivery.id,
                delivery: delivery_response,
                user: user_response,
                date: utils_delivery.date.and_utc(),
                observations: utils_delivery.observations,
            });
        }

        Ok(result)
    }

    pub async fn get_utils_deliveries_by_delivery(
        db: &DatabaseConnection,
        delivery_id: Uuid,
    ) -> Result<Vec<UtilsDeliveryWithDelivery>, DbErr> {
        // Find utils_deliveries for this delivery
        let utils_deliveries = Entity::find()
            .filter(entity::utils_delivery::Column::DeliveryId.eq(delivery_id))
            .all(db)
            .await?;

        let delivery = match delivery::Entity::find_by_id(delivery_id).one(db).await? {
            Some(d) => d,
            None => return Ok(vec![]),
        };

        let student = match student::Entity::find_by_id(delivery.student_id)
            .one(db)
            .await?
        {
            Some(s) => s,
            None => return Ok(vec![]),
        };

        let mut result = Vec::new();

        for utils_delivery in utils_deliveries {
            let user = match user::Entity::find_by_id(utils_delivery.user_id)
                .one(db)
                .await?
            {
                Some(u) => u,
                None => continue, // Skip if user not found
            };

            let delivery_response = DeliveryWithStudent {
                delivery: DeliveryResponse {
                    id: delivery.id,
                    status: delivery.state.clone(),
                },
                student: Some(student.clone()),
            };

            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                last_name: user.last_name,
                email: user.email,
                photo_url: user.photo_url,
                role: user.role,
            };

            result.push(UtilsDeliveryWithDelivery {
                id: utils_delivery.id,
                delivery: delivery_response,
                user: user_response,
                date: utils_delivery.date.and_utc(),
                observations: utils_delivery.observations,
            });
        }

        Ok(result)
    }

    pub async fn create_utils_delivery(
        db: &DatabaseConnection,
        delivery_dto: CreateUtilsDeliveryDto,
    ) -> Result<Model, DbErr> {
        let utils_delivery = ActiveModel {
            id: Set(Uuid::new_v4()),
            delivery_id: Set(delivery_dto.delivery),
            user_id: Set(delivery_dto.user),
            observations: Set(delivery_dto.observations),
            date: Set(delivery_dto.date.naive_utc()),
        };

        utils_delivery.insert(db).await
    }

    pub async fn update_utils_delivery(
        db: &DatabaseConnection,
        id: Uuid,
        delivery_dto: UpdateUtilsDeliveryDto,
    ) -> Result<Model, DbErr> {
        let utils_delivery = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_delivery) = utils_delivery {
            let mut delivery_active_model: ActiveModel = existing_delivery.into();

            if let Some(delivery_id) = delivery_dto.delivery {
                delivery_active_model.delivery_id = Set(delivery_id);
            }

            if let Some(user_id) = delivery_dto.user {
                delivery_active_model.user_id = Set(user_id);
            }

            if let Some(delivery_date) = delivery_dto.date {
                delivery_active_model.date = Set(delivery_date.naive_utc());
            }

            if let Some(observations) = delivery_dto.observations {
                delivery_active_model.observations = Set(observations);
            }

            delivery_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound("UtilsDelivery not found".to_string()))
        }
    }

    pub async fn delete_utils_delivery(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
