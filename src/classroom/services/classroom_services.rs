use super::super::dtos::create_classroom::ClassroomDto;
use entity::classroom::{ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

pub struct ClassroomService;

impl ClassroomService {
    pub async fn get_classroom(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn create_classroom(
        db: &DatabaseConnection,
        classroom_dto: ClassroomDto,
    ) -> Result<Model, DbErr> {
        let classroom = ActiveModel {
            name: Set(classroom_dto.name),
            ..Default::default()
        };

        classroom.insert(db).await
    }

    pub async fn update_classroom(
        db: &DatabaseConnection,
        id: i32,
        classroom_dto: ClassroomDto,
    ) -> Result<Model, DbErr> {
        let classroom = Entity::find_by_id(id).one(db).await?;

        if let Some(classroom) = classroom {
            let mut classroom: ActiveModel = classroom.into();
            classroom.name = Set(classroom_dto.name);
            classroom.utils_list_id = Set(classroom_dto.utils_list_id);

            classroom.update(db).await
        } else {
            Err(DbErr::RecordNotFound("Classroom not found".to_string()))
        }
    }

    pub async fn delete_classroom(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
