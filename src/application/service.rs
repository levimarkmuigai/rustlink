use crate::domain::{
    errors::LinkError,
    link::{Link, LinkId},
};

#[derive(Debug, Clone, PartialEq)]
pub struct LinkService;

impl LinkService {
    pub fn create() -> Result<Link, LinkError> {
        let link_id = LinkId::value();

        Ok(())
    }
}

