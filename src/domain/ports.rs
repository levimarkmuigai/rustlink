use crate::domain::{
    errors::LinkError,
    link::{Link, LinkHashedCode, LinkId},
};

use async_trait::async_trait;

#[async_trait]
pub trait LinkPersistence: Send + Sync {
    async fn delete_by_id(&self, id: LinkId) -> Result<Option<Link>, LinkError>;
    async fn save(&self, link: Link) -> Result<LinkId, LinkError>;
}

#[async_trait]
pub trait LinkQuery: Send + Sync {
    async fn find_by_id(&self, id: LinkId) -> Result<Link, LinkError>;
    async fn find_hashed_code(&self, id: LinkId) -> Result<LinkHashedCode, LinkError>;
}
