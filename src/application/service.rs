use crate::application::{
    command::Url,
    usecase::{LinkPersistenceService, LinkQueryService},
};
use crate::domain::{
    errors::LinkError,
    link::{CreatedAt, Link, LinkHashedCode, LinkId, ShortUrl},
    ports::{LinkPersistence, LinkQuery},
};

#[derive(Debug, Clone, PartialEq)]
pub struct LinkService<P: LinkPersistence, Q: LinkQuery> {
    persistence_service: LinkPersistenceService<P>,
    query_service: LinkQueryService<Q>,
}

impl<P, Q> LinkService<P, Q>
where
    P: LinkPersistence + Send + Sync,
    Q: LinkQuery + Send + Sync,
{
    pub fn new(persistence: LinkPersistenceService<P>, query: LinkQueryService<Q>) -> Self {
        Self {
            persistence_service: persistence,
            query_service: query,
        }
    }

    pub fn create(&self, raw_user_url: String) -> Result<LinkId, LinkError> {
        let link_uuid = LinkId::value();
        let delete_key = LinkHashedCode::value()?;
        let generated_url = ShortUrl::value()?;
        let creation_time = CreatedAt::value();

        let user_url = Url::new(raw_user_url).map_err(|_| LinkError::EmptyURL)?;

        let link = Link::new(
            link_uuid,
            delete_key.clone(),
            generated_url.clone().into_inner(),
            user_url.as_str().to_string(),
            creation_time,
        )
        .map_err(|_| LinkError::LinkCreationError)?;

        self.persistence_service
            .save(link)
            .map_err(|e| LinkError::PersistenceError(e.to_string()))?;

        Ok(LinkId::from(link_uuid))
    }

    pub fn delete(&self, id: LinkId) -> Result<Option<Link>, LinkError> {
        let link = self
            .query_service
            .find_by_id(id)
            .map_err(|e| LinkError::PersistenceError(e.to_string()))?;

        let link_id = link.id();

        self.persistence_service
            .delete_by_id(link_id.clone())
            .map_err(|e| LinkError::PersistenceError(e.to_string()))?;

        Ok(Some(link))
    }
}
