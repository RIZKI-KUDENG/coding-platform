use sqlx::PgPool;

use crate::modules::identity::domain::entities::user::User;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                email,
                username,
                password,
                created_at,
                updated_at
            FROM identity.users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
    }
}
