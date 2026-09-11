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

    pub async fn find_by_email_or_username(
        &self,
        identifier: &str,
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
            WHERE email = $1 OR username = $1
            "#,
            identifier
        )
        .fetch_optional(&self.pool)
        .await
    }


    pub async fn create_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO identity.users(email, username, password)
            VALUES ($1, $2, $3)
            RETURNING id, email, username, password, created_at, updated_at
            "#,
            email,
            username,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
    }
}
