use crate::application::usecase::LinkPersistenceService;
use crate::domain::{
    errors::LinkError,
    link::{CreatedAt, Link, LinkHashedCode, LinkId, ShortUrl, UserUrl},
    ports::LinkPersistence,
};

#[derive(Debug, Clone, PartialEq)]
pub struct LinkService<P: LinkPersistence> {
    persistence_service: LinkPersistenceService<P>,
}

impl<P> LinkService<P>
where
    P: LinkPersistence + Send + Sync,
{
    pub fn new(persistence: LinkPersistenceService<P>) -> Self {
        Self {
            persistence_service: persistence,
        }
    }

    pub fn create(&self, raw_user_url: String) -> Result<LinkId, LinkError> {
        let link_uuid = LinkId::value();
        let delete_key = LinkHashedCode::value()?;
        let generated_url = ShortUrl::value()?;
        let creation_time = CreatedAt::value();

        let user_url = UserUrl::try_from(raw_user_url).map_err(|_| LinkError::EmptyURL)?;

        let link = Link::new(
            link_uuid,
            delete_key.clone(),
            generated_url.clone().into_inner(),
            user_url.value().clone(),
            creation_time,
        )
        .map_err(|_| LinkError::CodeGenerationFailure)?;

        self.persistence_service
            .save(link)
            .map_err(|e| LinkError::PersistenceError(e.to_string()))?;

        Ok(LinkId::from(link_uuid))
    }
}
