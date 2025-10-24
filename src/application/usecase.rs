use crate::domain::{errors::LinkError, link::{Link, LinkId}, ports::{LinkPersistence, LinkQuery}};

pub struct LinkPersistenceService <P: LinkPersistence> {
    persistence: P,
}

impl <P: LinkPersistence> LinkPersistenceService<P> {

    pub fn new(persistence: P) -> Self { Self { persistence }}

    pub fn save(&self, link: Link) -> Result<Link, LinkError> {
        self.persistence.save(link)
    }

    pub fn delete_by_id(&self, id: LinkId) -> Result<Option<Link>, LinkError> {
        self.persistence.delete_by_id(id)
    }
}

pub struct LinkQueryService <Q: LinkQuery> {
    query: Q,
}

impl <Q: LinkQuery> LinkQueryService<Q> {
    
    pub fn new(query: Q) -> Self{ Self { query }}

    pub fn find_by_id(&self, id: LinkId) -> Result<Link, LinkError> {
        self.query.find_by_id(id)
    }

    pub fn find_by_short_code(&self, short_code: &str) -> Result<Link, LinkError> {
        self.query.find_by_short_code(short_code)
    }
}