use crate::domain::{
    errors::LinkError,
    link::{Link, LinkId, LinkKey, ShortUrl},
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
    async fn find_delete_key(&self, id: LinkId) -> Result<LinkKey, LinkError>;
    async fn find_by_short_code(&self, short_code: ShortUrl) -> Result<Link, LinkError>;
}
