use entity::purchase::{ActiveModel, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::purchase::dtos::{CreatePurchaseDto, PurchaseResponse, UpdatePurchaseDto};

pub struct PurchaseService;

impl PurchaseService {
    pub async fn get_purchase(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<PurchaseResponse>, DbErr> {
        let result = Entity::find_by_id(id).one(db).await?;
        Ok(result.map(|purchase| PurchaseResponse {
            id: purchase.id,
            total_spent: purchase.total_spent,
            date: purchase.date.and_utc(),
            user_id: purchase.user_id,
        }))
    }

    pub async fn get_all_purchases(
        db: &DatabaseConnection,
    ) -> Result<Vec<PurchaseResponse>, DbErr> {
        let results = Entity::find().all(db).await?;

        let purchases = results
            .into_iter()
            .map(|purchase| PurchaseResponse {
                id: purchase.id,
                total_spent: purchase.total_spent,
                date: purchase.date.and_utc(),
                user_id: purchase.user_id,
            })
            .collect();

        Ok(purchases)
    }

    pub async fn get_purchases_by_user(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<PurchaseResponse>, DbErr> {
        let results = Entity::find()
            .filter(entity::purchase::Column::UserId.eq(user_id))
            .all(db)
            .await?;

        let purchases = results
            .into_iter()
            .map(|purchase| PurchaseResponse {
                id: purchase.id,
                total_spent: purchase.total_spent,
                date: purchase.date.and_utc(),
                user_id: purchase.user_id,
            })
            .collect();

        Ok(purchases)
    }

    pub async fn create_purchase(
        db: &DatabaseConnection,
        purchase_dto: CreatePurchaseDto,
    ) -> Result<Model, DbErr> {
        let purchase = ActiveModel {
            id: Set(Uuid::new_v4()),
            total_spent: Set(purchase_dto.total_spent),
            date: Set(purchase_dto.date.naive_utc()),
            user_id: Set(purchase_dto.user_id),
        };

        purchase.insert(db).await
    }

    pub async fn update_purchase(
        db: &DatabaseConnection,
        id: Uuid,
        purchase_dto: UpdatePurchaseDto,
    ) -> Result<Model, DbErr> {
        let purchase = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_purchase) = purchase {
            let mut purchase_active_model: ActiveModel = existing_purchase.into();

            if let Some(total_spent) = purchase_dto.total_spent {
                purchase_active_model.total_spent = Set(total_spent);
            }

            if let Some(date) = purchase_dto.date {
                purchase_active_model.date = Set(date.naive_utc());
            }

            if let Some(user_id) = purchase_dto.user_id {
                purchase_active_model.user_id = Set(user_id);
            }

            purchase_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound("Purchase not found".to_string()))
        }
    }

    pub async fn delete_purchase(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
