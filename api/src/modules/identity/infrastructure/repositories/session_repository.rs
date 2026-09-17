use crate::modules::identity::domain::entities::session::Session;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        token_hash: String,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO identity.sessions(
                user_id,
                token_hash,
                expires_at
                )
                VALUES ($1, $2, $3)
            "#,
            user_id,
            token_hash,
            expires_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_token(&self, token_hash: &str) -> Result<Option<Session>, sqlx::Error> {
        let result = sqlx::query_as!(
            Session,
            r#"
                SELECT * FROM identity.sessions
                WHERE token_hash = $1
            "#,
            token_hash,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
    pub async fn find_user_by_valid_token(
        &self,
        token_hash: &str,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT user_id
            FROM identity.sessions
            WHERE token_hash = $1
            AND expires_at > NOW()
            "#,
            token_hash
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.user_id))
    }
}
