use crate::modules::system::domain::entities::feature_flag::FeatureFlag;
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FeatureFlagRepository {
    pool: PgPool,
}

impl FeatureFlagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<FeatureFlag>, sqlx::Error> {
        sqlx::query_as!(
            FeatureFlag,
            "SELECT id,
            key,
            is_enabled,
            description,
            created_at,
            updated_at
            FROM system.m_feature_flags
            ORDER BY key ASC
            "
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_flags_map(&self) -> Result<HashMap<String, bool>, sqlx::Error> {
        let flags = self.get_all().await?;
        let mut map = HashMap::new();
        for flag in flags {
            let is_active = flag.is_active();
            map.insert(flag.key, is_active);
        }
        Ok(map)
    }
    pub async fn is_enabled(&self, key: &str) -> Result<Option<bool>, sqlx::Error> {
        let row = sqlx::query!(
            r#"SELECT is_enabled FROM system.m_feature_flags WHERE key = $1"#,
            key
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| r.is_enabled == 1))
    }
}
