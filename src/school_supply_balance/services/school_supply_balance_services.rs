use entity::school_supply_balance::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::school_supply_balance::dtos::{
    CreateSchoolSupplyBalanceDto, UpdateSchoolSupplyBalanceDto,
};

pub struct SchoolSupplyBalanceService;

impl SchoolSupplyBalanceService {
    pub async fn get_school_supply_balance(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn get_all_school_supply_balances(
        db: &DatabaseConnection,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }

    pub async fn create_school_supply_balance(
        db: &DatabaseConnection,
        balance_dto: CreateSchoolSupplyBalanceDto,
    ) -> Result<Model, DbErr> {
        let balance = ActiveModel {
            id: Set(Uuid::new_v4()),
            balance: Set(balance_dto.balance),
        };

        balance.insert(db).await
    }

    pub async fn update_school_supply_balance(
        db: &DatabaseConnection,
        id: Uuid,
        balance_dto: UpdateSchoolSupplyBalanceDto,
    ) -> Result<Model, DbErr> {
        let balance = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_balance) = balance {
            let mut balance_active_model: ActiveModel = existing_balance.into();

            if let Some(balance) = balance_dto.balance {
                balance_active_model.balance = Set(balance);
            }

            balance_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound(
                "School Supply Balance not found".to_string(),
            ))
        }
    }

    pub async fn delete_school_supply_balance(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
