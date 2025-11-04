use crate::domain::{
    errors::LinkError,
    link::{Link, LinkHashedCode, LinkId},
};

#[async_trait::async_trait]
pub trait LinkPersistence: Send + Sync {
    fn delete_by_id(&self, id: LinkId) -> Result<Option<Link>, LinkError>;

    fn save(&self, link: Link) -> Result<LinkId, LinkError>;
}

#[async_trait::async_trait]
pub trait LinkQuery: Send + Sync {
    fn find_by_id(&self, id: LinkId) -> Result<Link, LinkError>;

    fn find_by_short_code(&self, short_code: &str) -> Result<Link, LinkError>;

    fn find_hashed_code(&self, id: LinkId) -> Result<LinkHashedCode, LinkError>;
}
