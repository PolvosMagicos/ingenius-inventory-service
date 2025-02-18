use entity::{
    classroom::{ActiveModel, Entity, Model},
    utils_list,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::classroom::dtos::{
    ClassroomResponse, ClassroomWithUtilsList, CreateClassroomDto, UpdateClassroomDto,
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
        Ok(result.map(|(classroom, utils_list)| {
            let classroom_response = ClassroomResponse {
                id: classroom.id,
                name: classroom.name,
            };
            ClassroomWithUtilsList {
                classroom: classroom_response,
                utils_list,
            }
        }))
    }

    pub async fn get_all_classrooms(
        db: &DatabaseConnection,
    ) -> Result<Vec<ClassroomWithUtilsList>, DbErr> {
        let results = Entity::find()
            .find_also_related(utils_list::Entity)
            .all(db)
            .await?;

        let classrooms_with_utils = results
            .into_iter()
            .map(|(classroom, utils_list)| {
                let classroom_response = ClassroomResponse {
                    id: classroom.id,
                    name: classroom.name,
                };
                ClassroomWithUtilsList {
                    classroom: classroom_response,
                    utils_list,
                }
            })
            .collect();
        Ok(classrooms_with_utils)
    }

    pub async fn create_classroom(
        db: &DatabaseConnection,
        classroom_dto: CreateClassroomDto,
    ) -> Result<Model, DbErr> {
        let classroom = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(classroom_dto.name),
            utils_list_id: Set(classroom_dto.utils_list),
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
