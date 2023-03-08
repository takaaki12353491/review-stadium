use async_trait::async_trait;
use common::error::DomainError;
use mockall::automock;

use crate::domain::user::User;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &String) -> Result<Option<User>, DomainError>;
}
