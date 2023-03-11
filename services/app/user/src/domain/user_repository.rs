use async_trait::async_trait;
use common::{error::DomainError, model::ID};
use mockall::automock;

use crate::domain::user::User;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &ID) -> Result<Option<User>, DomainError>;
}
