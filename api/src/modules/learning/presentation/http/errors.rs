//! Maps learning application errors to the public API error envelope.

use crate::modules::learning::application::use_cases::{
    admin::course::commands::{
        delete_course_admin::DeleteCourseError, edit_course_admin::EditCourseError,
    },
    course::queries::{
        get_all_course::GetAllCourseError, get_course_by_id::GetCourseByIdError,
        get_course_by_slug::GetCourseBySlugError,
    },
    exercise::queries::{
        get_exercise_by_id::GetExerciseByIdError,
        get_exercise_by_lesson_id::GetExerciseByLessonIdError,
    },
    lesson::queries::{
        get_lesson_by_course_id::GetLessonByCourseIdError, get_lesson_by_id::GetLessonByIdError,
        get_lesson_by_slug::GetLessonBySlugError,
    },
};
use crate::modules::shared::http::ApiError;

fn course_not_found() -> ApiError {
    ApiError::not_found("COURSE_NOT_FOUND", "Course not found.")
}

fn lesson_not_found() -> ApiError {
    ApiError::not_found("LESSON_NOT_FOUND", "Lesson not found.")
}

fn exercise_not_found() -> ApiError {
    ApiError::not_found("EXERCISE_NOT_FOUND", "Exercise not found.")
}

impl From<GetAllCourseError> for ApiError {
    fn from(err: GetAllCourseError) -> Self {
        match err {
            GetAllCourseError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<GetCourseByIdError> for ApiError {
    fn from(err: GetCourseByIdError) -> Self {
        match err {
            GetCourseByIdError::CourseNotFound => course_not_found(),
            GetCourseByIdError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<GetCourseBySlugError> for ApiError {
    fn from(err: GetCourseBySlugError) -> Self {
        match err {
            GetCourseBySlugError::CourseNotFound => course_not_found(),
            GetCourseBySlugError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<EditCourseError> for ApiError {
    fn from(err: EditCourseError) -> Self {
        match err {
            EditCourseError::CourseNotFound => course_not_found(),
            EditCourseError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<DeleteCourseError> for ApiError {
    fn from(err: DeleteCourseError) -> Self {
        match err {
            DeleteCourseError::CourseNotFound => course_not_found(),
            DeleteCourseError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<GetLessonByCourseIdError> for ApiError {
    fn from(err: GetLessonByCourseIdError) -> Self {
        match err {
            GetLessonByCourseIdError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<GetLessonByIdError> for ApiError {
    fn from(err: GetLessonByIdError) -> Self {
        match err {
            GetLessonByIdError::NotFound => lesson_not_found(),
            GetLessonByIdError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<GetLessonBySlugError> for ApiError {
    fn from(err: GetLessonBySlugError) -> Self {
        match err {
            GetLessonBySlugError::NotFound => lesson_not_found(),
            GetLessonBySlugError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<GetExerciseByIdError> for ApiError {
    fn from(err: GetExerciseByIdError) -> Self {
        match err {
            GetExerciseByIdError::NotFound => exercise_not_found(),
            GetExerciseByIdError::DatabaseError(e) => ApiError::internal(&e),
        }
    }
}

impl From<GetExerciseByLessonIdError> for ApiError {
    fn from(err: GetExerciseByLessonIdError) -> Self {
        match err {
            GetExerciseByLessonIdError::NotFound => exercise_not_found(),
            GetExerciseByLessonIdError::InternalServerError(e) => ApiError::internal(&e),
        }
    }
}
