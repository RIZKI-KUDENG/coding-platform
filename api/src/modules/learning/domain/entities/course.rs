use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Course {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Course {
    pub fn update(
        &mut self,
        title: String,
        slug: String,
        description: Option<String>,
        status: String,
    ) {
        self.title = title;
        self.slug = slug;
        self.description = description;
        self.status = status;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_update() {
        let now = Utc::now();
        let mut course = Course {
            id: Uuid::new_v4(),
            title: "Old Title".to_string(),
            slug: "old-slug".to_string(),
            description: Some("Old desc".to_string()),
            status: "DRAFT".to_string(),
            created_at: now,
            updated_at: now,
        };

        course.update(
            "New Title".to_string(),
            "new-slug".to_string(),
            Some("New desc".to_string()),
            "PUBLISHED".to_string(),
        );

        assert_eq!(course.title, "New Title");
        assert_eq!(course.slug, "new-slug");
        assert_eq!(course.description.as_deref(), Some("New desc"));
        assert_eq!(course.status, "PUBLISHED");
        assert!(course.updated_at >= now);
    }
}
