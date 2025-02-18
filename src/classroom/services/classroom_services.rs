use entity::classroom::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::classroom::dtos::{CreateClassroomDto, UpdateClassroomDto};

pub struct ClassroomService;

impl ClassroomService {
    pub async fn get_classroom(db: &DatabaseConnection, id: Uuid) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn get_all_classrooms(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }

    pub async fn create_classroom(
        db: &DatabaseConnection,
        classroom_dto: CreateClassroomDto,
    ) -> Result<Model, DbErr> {
        let classroom = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(classroom_dto.name),
            ..Default::default()
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

            classroom_active_model.utils_list_id = Set(classroom_dto.utils_list_id);

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
