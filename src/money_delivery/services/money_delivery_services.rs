use entity::{
    delivery,
    money_delivery::{ActiveModel, Entity, Model},
    user,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::{
    money_delivery::dtos::{
        create_money_delivery::CreateMoneyDeliveryDto,
        response::{MoneyDeliveryResponse, MoneyDeliveryWithUser},
        update_money_delivery::UpdateMoneyDeliveryDto,
    },
    user::dtos::response::UserResponse,
};

pub struct MoneyDeliveryService;

impl MoneyDeliveryService {
    pub async fn get_money_delivery(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<MoneyDeliveryWithUser>, DbErr> {
        let result = Entity::find_by_id(id)
            .find_also_related(delivery::Entity)
            .one(db)
            .await?;

        if let Some((money_delivery, _delivery)) = result {
            let user = user::Entity::find_by_id(money_delivery.user_id)
                .one(db)
                .await?;

            if let Some(user) = user {
                let user_response = UserResponse {
                    id: user.id,
                    name: user.name,
                    last_name: user.last_name,
                    email: user.email,
                    photo_url: user.photo_url,
                    role: user.role,
                };

                let money_response = MoneyDeliveryResponse {
                    id: money_delivery.id,
                    amount: money_delivery.amount,
                    date: money_delivery.date.and_utc(),
                    observations: money_delivery.observations,
                    delivery_id: money_delivery.delivery_id,
                };

                return Ok(Some(MoneyDeliveryWithUser {
                    money_delivery: money_response,
                    user: user_response,
                }));
            }
        }
        Ok(None)
    }

    pub async fn get_all_money_deliveries(
        db: &DatabaseConnection,
    ) -> Result<Vec<MoneyDeliveryWithUser>, DbErr> {
        let results = Entity::find()
            .find_also_related(user::Entity)
            .all(db)
            .await?;

        let money_deliveries_with_details = results
            .into_iter()
            .map(|(money_delivery, user)| {
                let money_response = MoneyDeliveryResponse {
                    id: money_delivery.id,
                    amount: money_delivery.amount,
                    date: money_delivery.date.and_utc(),
                    observations: money_delivery.observations,
                    delivery_id: money_delivery.delivery_id,
                };
                MoneyDeliveryWithUser {
                    money_delivery: money_response,
                    user: UserResponse {
                        id: user.clone().unwrap().id,
                        name: user.clone().unwrap().name,
                        last_name: user.clone().unwrap().last_name,
                        email: user.clone().unwrap().email,
                        photo_url: user.clone().unwrap().photo_url,
                        role: user.clone().unwrap().role,
                    },
                }
            })
            .collect();
        Ok(money_deliveries_with_details)
    }

    pub async fn create_money_delivery(
        db: &DatabaseConnection,
        money_delivery_dto: CreateMoneyDeliveryDto,
    ) -> Result<Model, DbErr> {
        let money_delivery = ActiveModel {
            id: Set(Uuid::new_v4()),
            amount: Set(money_delivery_dto.amount),
            date: Set(money_delivery_dto.date.naive_utc()),
            observations: Set(money_delivery_dto.observations.unwrap()),
            delivery_id: Set(money_delivery_dto.delivery_id),
            user_id: Set(money_delivery_dto.user_id),
        };

        money_delivery.insert(db).await
    }

    pub async fn update_money_delivery(
        db: &DatabaseConnection,
        id: Uuid,
        money_delivery_dto: UpdateMoneyDeliveryDto,
    ) -> Result<Model, DbErr> {
        let money_delivery = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_money_delivery) = money_delivery {
            let mut money_delivery_active_model: ActiveModel = existing_money_delivery.into();

            if let Some(amount) = money_delivery_dto.amount {
                money_delivery_active_model.amount = Set(amount);
            }

            if let Some(date) = money_delivery_dto.date {
                money_delivery_active_model.date = Set(date.naive_utc());
            }

            if let Some(observations) = money_delivery_dto.observations {
                money_delivery_active_model.observations = Set(observations);
            }

            if let Some(delivery_id) = money_delivery_dto.delivery_id {
                money_delivery_active_model.delivery_id = Set(delivery_id);
            }

            if let Some(user_id) = money_delivery_dto.user_id {
                money_delivery_active_model.user_id = Set(user_id);
            }

            money_delivery_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound(
                "Money delivery not found".to_string(),
            ))
        }
    }

    pub async fn delete_money_delivery(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
