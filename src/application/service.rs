use crate::domain::errors::LinkError;
use crate::domain::link::Link;
use crate::domain::ports::LinkRepository;

use super::command::{CreateLinkCommand, LinkCreationResult};

pub struct LinkService<R: LinkRepository> {
    repo: R,
}

impl<R: LinkRepository> LinkService<R> {
    pub fn new(repo: R) -> Self {
        LinkService { repo }
    }

    pub async fn create_new_link(
        &self,
        command: CreateLinkCommand,
    ) -> Result<LinkCreationResult, LinkError> {
    }
}
