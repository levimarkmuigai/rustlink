use crate::domain::{
    errors::LinkError,
    link::{Link, LinkId},
    ports::LinkPersistence,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::types::time::OffsetDateTime;
use sqlx::PgPool;

pub struct PostgresLinkRepository {
    pool: PgPool,
}

impl PostgresLinkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LinkPersistence for PostgresLinkRepository {
    async fn delete_by_id(&self, id: LinkId) -> Result<Option<Link>, LinkError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM links
            WHERE id = $1
            RETURNING id, delete_key_hash, short_code, long_url, created_at
            "#,
            id.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LinkError::PersistenceError(e.to_string()))?;

        match result {
            Some(record) => {
                let created_at_utc =
                    DateTime::<Utc>::from_timestamp(record.created_at.unix_timestamp(), 0)
                        .ok_or_else(|| LinkError::PersistenceError("Invalid timestamp".into()))?;

                let link = Link::new(
                    record.id,
                    record.delete_key_hash,
                    record.short_code,
                    record.long_url,
                    created_at_utc,
                )
                .map_err(|_| LinkError::LinkCreationError)?;
                Ok(Some(link))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, link: Link) -> Result<LinkId, LinkError> {
        let id = link.id().clone().into_inner();
        let delete_key_hash = link.delete_hash_code().clone().into_inner();
        let short_code = link.short_url().clone().into_inner();
        let long_url = link.user_url().clone().into_inner();
        let created_at_chrono = link.clone().created_at().into_inner();

        // Convert chrono::DateTime<Utc> -> time::OffsetDateTime
        let created_at: OffsetDateTime =
            OffsetDateTime::from_unix_timestamp(created_at_chrono.timestamp())
                .map_err(|_| LinkError::PersistenceError("Invalid timestamp conversion".into()))?;

        sqlx::query!(
            r#"
            INSERT INTO links (id, delete_key_hash, short_code, long_url, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            id,
            delete_key_hash,
            short_code,
            long_url,
            created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| LinkError::PersistenceError(e.to_string()))?;

        Ok(LinkId::from(id))
    }
}
