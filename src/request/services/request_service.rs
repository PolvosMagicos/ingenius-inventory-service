use entity::{
    classroom::Entity as ClassroomEntity,
    request::{ActiveModel, Entity as RequestEntity, Model},
    user::Entity as UserEntity,
    utils_list::Entity as UtilsListEntity,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::{
    classroom::dtos::{ClassroomResponse, ClassroomWithUtilsList},
    request::dtos::{
        create_request::CreateRequestDto, response::RequestResponse,
        update_request::UpdateRequestDto,
    },
    user::dtos::response::UserResponse,
};

pub struct RequestService;

impl RequestService {
    pub async fn get_request(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<RequestResponse>, DbErr> {
        let request_result = RequestEntity::find_by_id(id).one(db).await?;

        if let Some(request) = request_result {
            let classroom_result = ClassroomEntity::find_by_id(request.classroom_id)
                .one(db)
                .await?;

            let user_result = UserEntity::find_by_id(request.user_id).one(db).await?;

            let utils_list = if let Some(ref classroom) = classroom_result {
                UtilsListEntity::find_by_id(classroom.utils_list_id.unwrap_or_default())
                    .one(db)
                    .await?
            } else {
                None
            };

            let user_response = user_result.map(|u| UserResponse {
                id: u.id,
                name: u.name,
                last_name: u.last_name,
                email: u.email,
                photo_url: u.photo_url,
                role: u.role,
            });

            let classroom_with_utils = classroom_result.map(|c| ClassroomWithUtilsList {
                classroom: ClassroomResponse {
                    id: c.id,
                    name: c.name,
                },
                utils_list,
                user: user_response.clone(),
            });

            Ok(Some(RequestResponse {
                id: request.id.to_string(),
                justification: request.justification,
                date: request.date.and_utc(),
                status: request.status,
                classroom: classroom_with_utils.unwrap(),
                user: user_response.unwrap(),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_requests(db: &DatabaseConnection) -> Result<Vec<RequestResponse>, DbErr> {
        let requests = RequestEntity::find().all(db).await?;
        let mut response_list = Vec::new();

        for request in requests {
            let classroom_result = ClassroomEntity::find_by_id(request.classroom_id)
                .one(db)
                .await?;

            let user_result = UserEntity::find_by_id(request.user_id).one(db).await?;

            let utils_list = if let Some(ref classroom) = classroom_result {
                UtilsListEntity::find_by_id(classroom.utils_list_id.unwrap_or_default())
                    .one(db)
                    .await?
            } else {
                None
            };

            let user_response = user_result.map(|u| UserResponse {
                id: u.id,
                name: u.name,
                last_name: u.last_name,
                email: u.email,
                photo_url: u.photo_url,
                role: u.role,
            });

            let classroom_with_utils = classroom_result.map(|c| ClassroomWithUtilsList {
                classroom: ClassroomResponse {
                    id: c.id,
                    name: c.name,
                },
                utils_list,
                user: user_response.clone(),
            });

            response_list.push(RequestResponse {
                id: request.id.to_string(),
                justification: request.justification,
                date: request.date.and_utc(),
                status: request.status,
                classroom: classroom_with_utils.unwrap(),
                user: user_response.unwrap(),
            });
        }

        Ok(response_list)
    }

    pub async fn create_request(
        db: &DatabaseConnection,
        request_dto: CreateRequestDto,
    ) -> Result<Model, DbErr> {
        let request = ActiveModel {
            id: Set(Uuid::new_v4()),
            justification: Set(request_dto.justification),
            date: Set(request_dto.date.naive_utc()),
            status: Set(request_dto.status),
            classroom_id: Set(request_dto.classroom),
            user_id: Set(request_dto.user),
        };

        request.insert(db).await
    }

    pub async fn update_request(
        db: &DatabaseConnection,
        id: Uuid,
        request_dto: UpdateRequestDto,
    ) -> Result<Model, DbErr> {
        let request = RequestEntity::find_by_id(id).one(db).await?;

        if let Some(existing_request) = request {
            let mut request_active_model: ActiveModel = existing_request.into();

            if let Some(justification) = request_dto.justification {
                request_active_model.justification = Set(justification);
            }

            if let Some(date) = request_dto.date {
                request_active_model.date = Set(date.naive_utc());
            }

            if let Some(status) = request_dto.status {
                request_active_model.status = Set(status);
            }

            if let Some(user) = request_dto.user {
                request_active_model.user_id = Set(user);
            }

            if let Some(classroom) = request_dto.classroom {
                request_active_model.classroom_id = Set(classroom);
            }

            request_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound("Request not found".to_string()))
        }
    }

    pub async fn delete_request(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        RequestEntity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
