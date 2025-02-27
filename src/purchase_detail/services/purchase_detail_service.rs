use crate::purchase::dtos::PurchaseResponse;
use crate::purchase_detail::dtos::{
    CreatePurchaseDetailDto, PurchaseDetailResponse, UpdatePurchaseDetailDto,
};
use entity::{
    purchase,
    purchase_detail::{ActiveModel, Entity, Model},
    util,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

pub struct PurchaseDetailService;

impl PurchaseDetailService {
    pub async fn get_purchase_detail(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<PurchaseDetailResponse>, DbErr> {
        let result = Entity::find_by_id(id)
            .find_also_related(util::Entity)
            .one(db)
            .await?;

        if let Some((detail, util)) = result {
            let purchase = purchase::Entity::find_by_id(detail.purchase_id)
                .one(db)
                .await?;

            if let Some(purchase) = purchase {
                let purchase_response = PurchaseResponse {
                    id: purchase.id,
                    total_spent: purchase.total_spent,
                    date: purchase.date.and_utc(),
                    user_id: purchase.user_id,
                };

                return Ok(Some(PurchaseDetailResponse {
                    id: detail.id,
                    util: util.unwrap(),
                    purchase: purchase_response,
                    unit_price: detail.unit_price,
                    quantity: detail.quantity,
                }));
            }
        }
        Ok(None)
    }

    pub async fn get_all_purchase_details(
        db: &DatabaseConnection,
    ) -> Result<Vec<PurchaseDetailResponse>, DbErr> {
        let results = Entity::find()
            .find_also_related(util::Entity)
            .all(db)
            .await?;

        let mut details_responses = Vec::new();

        for (detail, util) in results {
            let purchase = purchase::Entity::find_by_id(detail.purchase_id)
                .one(db)
                .await?;

            if let Some(purchase) = purchase {
                let purchase_response = PurchaseResponse {
                    id: purchase.id,
                    total_spent: purchase.total_spent,
                    date: purchase.date.and_utc(),
                    user_id: purchase.user_id,
                };

                details_responses.push(PurchaseDetailResponse {
                    id: detail.id,
                    util: util.clone().unwrap(),
                    purchase: purchase_response,
                    unit_price: detail.unit_price,
                    quantity: detail.quantity,
                });
            }
        }
        Ok(details_responses)
    }

    pub async fn create_purchase_detail(
        db: &DatabaseConnection,
        detail_dto: CreatePurchaseDetailDto,
    ) -> Result<Model, DbErr> {
        let detail = ActiveModel {
            id: Set(Uuid::new_v4()),
            purchase_id: Set(detail_dto.purchase_id),
            util_id: Set(detail_dto.util_id),
            unit_price: Set(detail_dto.unit_price),
            quantity: Set(detail_dto.quantity),
        };
        detail.insert(db).await
    }

    pub async fn update_purchase_detail(
        db: &DatabaseConnection,
        id: Uuid,
        detail_dto: UpdatePurchaseDetailDto,
    ) -> Result<Model, DbErr> {
        let detail = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_detail) = detail {
            let mut detail_active_model: ActiveModel = existing_detail.into();

            if let Some(purchase_id) = detail_dto.purchase_id {
                detail_active_model.purchase_id = Set(purchase_id);
            }

            if let Some(util_id) = detail_dto.util_id {
                detail_active_model.util_id = Set(util_id);
            }

            if let Some(unit_price) = detail_dto.unit_price {
                detail_active_model.unit_price = Set(unit_price);
            }

            if let Some(quantity) = detail_dto.quantity {
                detail_active_model.quantity = Set(quantity);
            }

            detail_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound(
                "PurchaseDetail not found".to_string(),
            ))
        }
    }

    pub async fn delete_purchase_detail(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
