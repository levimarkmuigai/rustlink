use axum::async_trait;
use uuid::Uuid;

use crate::domain::link::Link;

pub type RepositoryError = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait LinkRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Link>, RepositoryError>;

    async fn find_link_by_code(&self, short_code: &str) -> Result<Option<Link>, RepositoryError>;

    async fn insert_link(
        &self,
        long_url: String,
        short_code: String,
        delete_key_harsh: String,
    ) -> Result<Option<Link>, RepositoryError>;

    async fn delete_by_id(&self, id: Uuid) -> Result<(), RepositoryError>;
}
