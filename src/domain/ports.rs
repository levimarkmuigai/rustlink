use crate::domain::errors::LinkError;
use uuid::Uuid;

use super::link::Link;

pub trait Port {
    fn find_by_id(&self, id: Uuid) -> Result<Link, LinkError>;
    fn save(&self, link: &Link) -> ();
    fn delete() -> ();
}
