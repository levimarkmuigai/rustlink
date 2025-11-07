use chrono::{DateTime, Utc};
use sqlx::types::time::OffsetDateTime;
use sqlx::PgPool;

use async_trait::async_trait;

use crate::domain::{
    errors::LinkError,
    link::{Link, LinkId, LinkKey, ShortUrl},
    ports::{LinkPersistence, LinkQuery},
};

#[derive(Clone, Debug)]
pub struct PgPoolRepository {
    pool: PgPool,
}

impl PgPoolRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn to_offset_dt(dt: DateTime<Utc>) -> Result<OffsetDateTime, LinkError> {
    let timestamp = dt.timestamp();

    OffsetDateTime::from_unix_timestamp(timestamp)
        .map_err(|_| LinkError::PersistenceError("Invalid timestamp conversion".into()))
}

fn to_chrono_dt(offset_dt: OffsetDateTime) -> Result<DateTime<Utc>, LinkError> {
    let timestamp = offset_dt.unix_timestamp();

    let chrono_dt = DateTime::from_timestamp(timestamp, 0)
        .ok_or_else(|| LinkError::PersistenceError("Invalid timestamp".into()))?;

    Ok(chrono_dt)
}

#[async_trait]
impl LinkPersistence for PgPoolRepository {
    async fn save(&self, link: Link) -> Result<LinkId, LinkError> {
        let id = link.id().clone().into_inner();
        let delete_key = link.delete_hash_code().clone().into_inner();
        let short_code = link.short_url().clone().into_inner();
        let long_url = link.user_url().clone().into_inner();
        let created_at = to_offset_dt(link.created_at().into_inner())?;

        sqlx::query!(
            r#"
            INSERT INTO links (id, delete_key, short_code, long_url, created_at)
            VALUES ($1,$2, $3, $4, $5)
            "#,
            id,
            delete_key,
            short_code,
            long_url,
            created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| LinkError::PersistenceError(e.to_string()))?;

        Ok(LinkId::from(id))
    }

    async fn delete_by_id(&self, id: LinkId) -> Result<Option<Link>, LinkError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM links
            WHERE id = $1
            RETURNING id, delete_key, short_code, long_url, created_at
            "#,
            id.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LinkError::PersistenceError(e.to_string()))?;

        match result {
            Some(record) => {
                let created_at_utc = to_chrono_dt(record.created_at)?;

                let link = Link::new(
                    record.id,
                    record.delete_key,
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
}

#[async_trait]
impl LinkQuery for PgPoolRepository {
    async fn find_by_id(&self, id: LinkId) -> Result<Link, LinkError> {
        sqlx::query!(
            r#"
            SELECT id, delete_key, short_code, long_url, created_at
            FROM links
            WHERE id = $1
            "#,
            id.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LinkError::PersistenceError(e.to_string()))?
        .ok_or(LinkError::LinkIdNotFound)
        .and_then(|row| {
            let created_at_utc = to_chrono_dt(row.created_at)?;

            Link::new(
                row.id,
                row.delete_key,
                row.short_code,
                row.long_url,
                created_at_utc,
            )
            .map_err(|_| LinkError::LinkCreationError)
        })
    }

    async fn find_delete_key(&self, id: LinkId) -> Result<LinkKey, LinkError> {
        sqlx::query!(
            r#"
            SELECT delete_key
            FROM links
            WHERE id = $1 
            "#,
            id.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LinkError::PersistenceError(e.to_string()))?
        .ok_or(LinkError::LinkIdNotFound)
        .map(|row| LinkKey::new(row.delete_key))
    }

    async fn find_by_short_code(&self, code: ShortUrl) -> Result<Link, LinkError> {
        sqlx::query!(
            r#"
            SELECT id,delete_key,short_code,long_url,created_at
            FROM links
            WHERE short_code = $1
            "#,
            code.into_inner()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LinkError::PersistenceError(e.to_string()))?
        .ok_or(LinkError::LinkIdNotFound)
        .and_then(|row| {
            let created_at_utc = to_chrono_dt(row.created_at)?;

            Link::new(
                row.id,
                row.delete_key,
                row.short_code,
                row.long_url,
                created_at_utc,
            )
            .map_err(|_| LinkError::LinkCreationError)
        })
    }
}
