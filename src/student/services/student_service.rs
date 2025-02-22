use entity::student::{ActiveModel, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::student::dtos::{CreateStudentDto, StudentResponse, UpdateStudentDto};

pub struct StudentService;

impl StudentService {
    pub async fn get_student(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<StudentResponse>, DbErr> {
        let result = Entity::find_by_id(id).one(db).await?;
        Ok(result.map(|student| StudentResponse {
            id: student.id,
            name: student.name,
            last_name: student.last_name,
        }))
    }

    pub async fn get_all_students(db: &DatabaseConnection) -> Result<Vec<StudentResponse>, DbErr> {
        let results = Entity::find().all(db).await?;

        let students = results
            .into_iter()
            .map(|student| StudentResponse {
                id: student.id,
                name: student.name,
                last_name: student.last_name,
            })
            .collect();

        Ok(students)
    }

    pub async fn get_students_by_classroom(
        db: &DatabaseConnection,
        classroom_id: Uuid,
    ) -> Result<Vec<StudentResponse>, DbErr> {
        let results = Entity::find()
            .filter(entity::student::Column::ClassroomId.eq(classroom_id))
            .all(db)
            .await?;

        let students = results
            .into_iter()
            .map(|student| StudentResponse {
                id: student.id,
                name: student.name,
                last_name: student.last_name,
            })
            .collect();

        Ok(students)
    }

    pub async fn create_student(
        db: &DatabaseConnection,
        student_dto: CreateStudentDto,
    ) -> Result<Model, DbErr> {
        let student = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(student_dto.name),
            last_name: Set(student_dto.last_name),
            classroom_id: Set(student_dto.classroom_id),
        };

        student.insert(db).await
    }

    pub async fn update_student(
        db: &DatabaseConnection,
        id: Uuid,
        student_dto: UpdateStudentDto,
    ) -> Result<Model, DbErr> {
        let student = Entity::find_by_id(id).one(db).await?;

        if let Some(existing_student) = student {
            let mut student_active_model: ActiveModel = existing_student.into();

            if let Some(name) = student_dto.name {
                student_active_model.name = Set(name);
            }

            if let Some(last_name) = student_dto.last_name {
                student_active_model.last_name = Set(last_name);
            }

            if let Some(classroom_id) = student_dto.classroom_id {
                student_active_model.classroom_id = Set(classroom_id);
            }

            student_active_model.update(db).await
        } else {
            Err(DbErr::RecordNotFound("Student not found".to_string()))
        }
    }

    pub async fn delete_student(db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
