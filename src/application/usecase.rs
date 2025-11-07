use crate::domain::{
    errors::LinkError,
    link::{Link, LinkId, LinkKey, ShortUrl},
    ports::{LinkPersistence, LinkQuery},
};

#[derive(Debug, Clone, PartialEq)]
pub struct LinkPersistenceService<P: LinkPersistence + Send + Sync> {
    persistence: P,
}

impl<P: LinkPersistence + Send + Sync> LinkPersistenceService<P> {
    pub fn new(persistence: P) -> Self {
        Self { persistence }
    }

    pub async fn save(&self, link: Link) -> Result<LinkId, LinkError> {
        self.persistence.save(link).await
    }

    pub async fn delete_by_id(&self, id: LinkId) -> Result<Option<Link>, LinkError> {
        self.persistence.delete_by_id(id).await
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkQueryService<Q: LinkQuery + Send + Sync> {
    query: Q,
}

impl<Q: LinkQuery + Send + Sync> LinkQueryService<Q> {
    pub fn new(query: Q) -> Self {
        Self { query }
    }

    pub async fn find_by_id(&self, id: LinkId) -> Result<Link, LinkError> {
        self.query.find_by_id(id).await
    }

    pub async fn find_delete_key(&self, id: LinkId) -> Result<LinkKey, LinkError> {
        self.query.find_delete_key(id).await
    }

    pub async fn find_by_short_code(&self, short_code: ShortUrl) -> Result<Link, LinkError> {
        self.query.find_by_short_code(short_code).await
    }
}
