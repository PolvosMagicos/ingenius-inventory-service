use entity::{
    delivery::{ActiveModel, Entity, Model},
    student,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::delivery::dtos::{
    CreateDeliveryDto, DeliveryResponse, DeliveryWithStudent, UpdateDeliveryDto,
};

pub struct DeliveryService;

impl DeliveryService {
    pub async fn get_delivery(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<DeliveryWithStudent>, DbErr> {
        let result = Entity::find_by_id(id)
            .find_also_related(student::Entity)
            .one(db)
            .await?;
        Ok(result.map(|(delivery, student)| {
            let delivery_response = DeliveryResponse {
                id: delivery.id,
                status: delivery.state,
            };
            DeliveryWithStudent {
                delivery: delivery_response,
                student,
            }
        }))
    }

    pub async fn get_all_deliveries(
        db: &DatabaseConnection,
    ) -> Result<Vec<DeliveryWithStudent>, DbErr> {
        let results = Entity::find()
            .find_also_related(student::Entity)
            .all(db)
            .await?;

        let deliveries_with_students = results
            .into_iter()
            .map(|(delivery, student)| {
                let delivery_response = DeliveryResponse {
                    id: delivery.id,
                    status: delivery.state,
                };
                DeliveryWithStudent {
                    delivery: delivery_response,
                    student,
                }
            })
            .collect();
        Ok(deliveries_with_students)
    }

    pub async fn create_delivery(
        db: &DatabaseConnection,
        delivery_dto: CreateDeliveryDto,
    ) -> Result<Model, DbErr> {
        let delivery = ActiveModel {
            id: Set(Uuid::new_v4()),
            student_id: Set(delivery_dto.student_id.unwrap()),
            state: Set(delivery_dto.status),
        };

        delivery.insert(db).await
    }

    pub async fn update_delivery(
        db: &DatabaseConnection,
        id: Uuid,
        delivery_dto: UpdateDeliveryDto,
    ) -> Result<Model, DbErr> {
        let delivery = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_delivery) = delivery {
            let mut delivery_active_model: ActiveModel = existing_delivery.into();

            if let Some(status) = delivery_dto.status {
                delivery_active_model.state = Set(status);
            }

            delivery_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound("Delivery not found".to_string()))
        }
    }

    pub async fn delete_delivery(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
