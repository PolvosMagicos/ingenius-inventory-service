use entity::{
    school_supply_balance::{ActiveModel, Entity, Model},
    school_supply,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::school_supply_balance::dtos::{
    SchoolSupplyBalanceResponse, SchoolSupplyBalanceWithSupply, CreateSchoolSupplyBalanceDto, UpdateSchoolSupplyBalanceDto,
};

pub struct SchoolSupplyBalanceService;

impl SchoolSupplyBalanceService {
    pub async fn get_school_supply_balance(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<SchoolSupplyBalanceWithSupply>, DbErr> {
        let result = Entity::find_by_id(id)
            .find_also_related(school_supply::Entity)
            .one(db)
            .await?;
        
        Ok(result.map(|(balance, supply)| {
            let balance_response = SchoolSupplyBalanceResponse {
                id: balance.id,
                quantity: balance.quantity,
            };
            SchoolSupplyBalanceWithSupply {
                balance: balance_response,
                supply,
            }
        }))
    }

    pub async fn get_all_school_supply_balances(
        db: &DatabaseConnection,
    ) -> Result<Vec<SchoolSupplyBalanceWithSupply>, DbErr> {
        let results = Entity::find()
            .find_also_related(school_supply::Entity)
            .all(db)
            .await?;

        let balances_with_supply = results
            .into_iter()
            .map(|(balance, supply)| {
                let balance_response = SchoolSupplyBalanceResponse {
                    id: balance.id,
                    quantity: balance.quantity,
                };
                SchoolSupplyBalanceWithSupply {
                    balance: balance_response,
                    supply,
                }
            })
            .collect();
        Ok(balances_with_supply)
    }

    pub async fn create_school_supply_balance(
        db: &DatabaseConnection,
        balance_dto: CreateSchoolSupplyBalanceDto,
    ) -> Result<Model, DbErr> {
        let balance = ActiveModel {
            id: Set(Uuid::new_v4()),
            quantity: Set(balance_dto.quantity),
            school_supply_id: Set(balance_dto.school_supply),
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

            if let Some(quantity) = balance_dto.quantity {
                balance_active_model.quantity = Set(quantity);
            }

            if let Some(school_supply) = balance_dto.school_supply {
                balance_active_model.school_supply_id = Set(Some(school_supply));
            }

            balance_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound("School Supply Balance not found".to_string()))
        }
    }

    pub async fn delete_school_supply_balance(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
