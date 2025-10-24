use crate::domain::{
    errors::LinkError,
    link::{Link, LinkId},
};

pub trait LinkPersistence {
    fn delete_by_id(&self, id: LinkId) -> Result<Option<Link>, LinkError>;

    fn save(&self, link: Link) -> Result<Link, LinkError>;
}

pub trait LinkQuery {
    fn find_by_id(&self, id: LinkId) -> Result<Link, LinkError>;

    fn find_by_short_code(&self, short_code: &str) -> Result<Link, LinkError>;
}
