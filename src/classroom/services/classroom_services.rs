use entity::{
    classroom::{ActiveModel, Entity, Model},
    user, utils_list,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::{
    classroom::dtos::{
        ClassroomResponse, ClassroomWithUtilsList, CreateClassroomDto, UpdateClassroomDto,
    },
    user::dtos::response::UserResponse,
};

pub struct ClassroomService;

impl ClassroomService {
    pub async fn get_classroom(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<ClassroomWithUtilsList>, DbErr> {
        let result = Entity::find_by_id(id)
            .find_also_related(utils_list::Entity)
            .one(db)
            .await?;

        if let Some((classroom, utils_list)) = result {
            let user_response = if let Some(user_id) = classroom.user_id {
                if let Some(user) = user::Entity::find_by_id(user_id).one(db).await? {
                    Some(UserResponse {
                        id: user.id,
                        name: user.name,
                        last_name: user.last_name,
                        email: user.email,
                        photo_url: user.photo_url,
                        role: user.role,
                    })
                } else {
                    None
                }
            } else {
                None
            };

            let classroom_response = ClassroomResponse {
                id: classroom.id,
                name: classroom.name,
            };

            return Ok(Some(ClassroomWithUtilsList {
                classroom: classroom_response,
                utils_list,
                user: user_response,
            }));
        }

        Ok(None)
    }

    pub async fn get_all_classrooms(
        db: &DatabaseConnection,
    ) -> Result<Vec<ClassroomWithUtilsList>, DbErr> {
        let results = Entity::find()
            .find_also_related(utils_list::Entity)
            .all(db)
            .await?;

        let mut classrooms_responses = Vec::new();

        for (classroom, utils_list) in results {
            let user_response = if let Some(user_id) = classroom.user_id {
                if let Some(user) = user::Entity::find_by_id(user_id).one(db).await? {
                    Some(UserResponse {
                        id: user.id,
                        name: user.name,
                        last_name: user.last_name,
                        email: user.email,
                        photo_url: user.photo_url,
                        role: user.role,
                    })
                } else {
                    None
                }
            } else {
                None
            };
            let classroom_response = ClassroomResponse {
                id: classroom.id,
                name: classroom.name,
            };
            classrooms_responses.push(ClassroomWithUtilsList {
                classroom: classroom_response,
                utils_list,
                user: user_response,
            })
        }
        Ok(classrooms_responses)
    }

    pub async fn create_classroom(
        db: &DatabaseConnection,
        classroom_dto: CreateClassroomDto,
    ) -> Result<Model, DbErr> {
        let classroom = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(classroom_dto.name),
            utils_list_id: Set(classroom_dto.utils_list),
            user_id: Set(classroom_dto.user),
        };

        classroom.insert(db).await
    }

    pub async fn update_classroom(
        db: &DatabaseConnection,
        id: Uuid,
        classroom_dto: UpdateClassroomDto,
    ) -> Result<Model, DbErr> {
        let classroom = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_classroom) = classroom {
            let mut classroom_active_model: ActiveModel = existing_classroom.into();

            if let Some(name) = classroom_dto.name {
                classroom_active_model.name = Set(name);
            }

            if let Some(utils_list) = classroom_dto.utils_list {
                classroom_active_model.utils_list_id = Set(Some(utils_list));
            }

            if let Some(user) = classroom_dto.user {
                classroom_active_model.user_id = Set(Some(user));
            }

            classroom_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound("Classroom not found".to_string()))
        }
    }

    pub async fn delete_classroom(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
