use crate::{
    auth::dtos::{AuthResponse, Claims, LoginUserDto, RegisterUserDto},
    user::dtos::response::UserResponse,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use entity::user::{ActiveModel, Column, Entity};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;
pub struct AuthService {
    db: DatabaseConnection,
    jwt_secret: String,
}
impl AuthService {
    pub fn new(db: DatabaseConnection, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }
    pub async fn register(&self, dto: RegisterUserDto) -> Result<AuthResponse, DbErr> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(dto.password.as_bytes(), &salt)
            .map_err(|_| DbErr::Custom("Password hashing failed".to_string()))?
            .to_string();

        let user = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(dto.name),
            last_name: Set(dto.last_name),
            email: Set(dto.email),
            password: Set(password_hash),
            photo_url: Set("".to_string()),
            role: Set("user".to_string()),
        };

        let user = user.insert(&self.db).await?;
        let token = self.generate_token(&user.id.to_string(), &user.role);

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id.to_string(),
                name: user.name,
                last_name: user.last_name,
                email: user.email,
                photo_url: user.photo_url,
                role: user.role,
            },
        })
    }

    pub async fn login(&self, dto: LoginUserDto) -> Result<Option<AuthResponse>, DbErr> {
        let user = Entity::find()
            .filter(Column::Email.eq(dto.email))
            .one(&self.db)
            .await?;

        if let Some(user) = user {
            // Parse the stored password hash
            let parsed_hash = PasswordHash::new(&user.password)
                .map_err(|_| DbErr::Custom("Invalid stored password hash".to_string()))?;

            // Verify password
            if Argon2::default()
                .verify_password(dto.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                let token = self.generate_token(&user.id.to_string(), &user.role);
                return Ok(Some(AuthResponse {
                    token,
                    user: UserResponse {
                        id: user.id.to_string(),
                        name: user.name,
                        last_name: user.last_name,
                        email: user.email,
                        photo_url: user.photo_url,
                        role: user.role,
                    },
                }));
            }
        }
        Ok(None)
    }

    fn generate_token(&self, user_id: &str, role: &str) -> String {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .unwrap()
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            role: role.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .unwrap()
    }
}
