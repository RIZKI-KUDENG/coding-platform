use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::learning::domain::entities::course::Course;

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct EditCourseRequest {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Course> for CourseResponse {
    fn from(course: Course) -> Self {
        Self {
            id: course.id,
            title: course.title,
            slug: course.slug,
            description: course.description,
            status: course.status,
            created_at: course.created_at,
            updated_at: course.updated_at,
        }
    }
}
