use std::collections::HashMap;
use uuid::Uuid;

use crate::modules::system::domain::entities::feature_flag::{FeatureFlag, FeatureFlagDetail};
use sqlx::PgPool;

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
            parent_id,
            created_at,
            updated_at
            FROM system.m_feature_flags
            ORDER BY key ASC
            "
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_flags_detail_map(
        &self,
    ) -> Result<HashMap<String, FeatureFlagDetail>, sqlx::Error> {
        let flags = self.get_all().await?;
        let id_status: HashMap<Uuid, bool> = flags.iter().map(|f| (f.id, f.is_active())).collect();

        let mut map = HashMap::new();
        for flag in &flags {
            let parent_active = flag
                .parent_id
                .and_then(|pid| id_status.get(&pid).copied())
                .unwrap_or(true);

            let active = flag.is_active() && parent_active;
            map.insert(
                flag.key.clone(),
                FeatureFlagDetail {
                    is_enabled: active,
                    is_sub_feature: flag.is_sub_feature(),
                    parent_id: flag.parent_id,
                },
            );
        }
        Ok(map)
    }

    pub async fn get_flags_map(&self) -> Result<HashMap<String, bool>, sqlx::Error> {
        let flags = self.get_all().await?;
        let id_status: HashMap<Uuid, bool> = flags.iter().map(|f| (f.id, f.is_active())).collect();

        let mut map = HashMap::new();
        for flag in &flags {
            let parent_active = flag
                .parent_id
                .and_then(|pid| id_status.get(&pid).copied())
                .unwrap_or(true);

            let active = flag.is_active() && parent_active;
            map.insert(flag.key.clone(), active);
        }
        Ok(map)
    }

    pub async fn is_enabled(&self, key: &str) -> Result<Option<bool>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT 
                f.is_enabled,
                p.is_enabled AS "parent_is_enabled?"
            FROM system.m_feature_flags f
            LEFT JOIN system.m_feature_flags p ON f.parent_id = p.id
            WHERE f.key = $1
            "#,
            key
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let self_active = r.is_enabled == 1;
            let parent_active = r.parent_is_enabled.map(|p| p == 1).unwrap_or(true);
            self_active && parent_active
        }))
    }

    pub async fn find_by_key(&self, key: &str) -> Result<Option<FeatureFlag>, sqlx::Error> {
        sqlx::query_as!(
            FeatureFlag,
            r#"
            SELECT id,
                   key,
                   is_enabled,
                   description,
                   parent_id,
                   created_at,
                   updated_at
            FROM system.m_feature_flags
            WHERE key = $1
            "#,
            key
        )
        .fetch_optional(&self.pool)
        .await
    }
}
