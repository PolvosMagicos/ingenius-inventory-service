use crate::{
    auth::dtos::RegisterUserDto,
    user::dtos::{response::UserResponse, update_user::UpdateUserDto},
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use entity::user::{ActiveModel, Entity};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn get_user(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Option<UserResponse>, DbErr> {
        let user = Entity::find_by_id(user_id).one(db).await?;

        match user {
            Some(user) => Ok(Some(UserResponse {
                id: user.id,
                name: user.name,
                last_name: user.last_name,
                email: user.email,
                photo_url: user.photo_url,
                role: user.role,
            })),
            None => Ok(None),
        }
    }

    pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<UserResponse>, DbErr> {
        let results = Entity::find().all(db).await?;
        let users = results
            .into_iter()
            .map(|user| UserResponse {
                id: user.id,
                name: user.name,
                last_name: user.last_name,
                email: user.email,
                photo_url: user.photo_url,
                role: user.role,
            })
            .collect();

        Ok(users)
    }

    pub async fn create_user(
        db: &DatabaseConnection,
        user: RegisterUserDto,
    ) -> Result<UserResponse, DbErr> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(user.password.as_bytes(), &salt)
            .map_err(|_| DbErr::Custom("Password hashing failed".to_string()))?
            .to_string();

        let new_user = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(user.name),
            last_name: Set(user.last_name),
            password: Set(password_hash),
            email: Set(user.email),
            photo_url: Set("".to_string()),
            role: Set("user".to_string()),
        };

        let inserted_user = new_user.insert(db).await?;
        Ok(UserResponse {
            id: inserted_user.id,
            name: inserted_user.name,
            last_name: inserted_user.last_name,
            email: inserted_user.email,
            photo_url: inserted_user.photo_url,
            role: inserted_user.role,
        })
    }

    pub async fn update_user(
        db: &DatabaseConnection,
        user_id: Uuid,
        update_dto: UpdateUserDto,
    ) -> Result<UserResponse, DbErr> {
        let user = Entity::find_by_id(user_id).one(db).await?;

        if let Some(existing_user) = user {
            let mut active_model: ActiveModel = existing_user.into();

            if let Some(name) = update_dto.name {
                active_model.name = Set(name);
            }
            if let Some(last_name) = update_dto.last_name {
                active_model.last_name = Set(last_name);
            }
            if let Some(email) = update_dto.email {
                active_model.email = Set(email);
            }
            if let Some(photo_url) = update_dto.photo_url {
                active_model.photo_url = Set(photo_url);
            }
            if let Some(role) = update_dto.role {
                active_model.role = Set(role);
            }

            let updated_user = active_model.update(db).await?;

            Ok(UserResponse {
                id: updated_user.id,
                name: updated_user.name,
                last_name: updated_user.last_name,
                email: updated_user.email,
                photo_url: updated_user.photo_url,
                role: updated_user.role,
            })
        } else {
            Err(DbErr::RecordNotFound("User not found".to_string()))
        }
    }

    pub async fn delete_user(db: &DatabaseConnection, user_id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(user_id).exec(db).await?;
        Ok(())
    }
}
