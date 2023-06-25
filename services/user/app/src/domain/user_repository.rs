use super::user::User;
use async_trait::async_trait;
use common::{error::DomainError, model::ID};
use mockall::automock;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &ID) -> Result<Option<User>, DomainError>;
    async fn find_by_id_name(&self, id_name: &str) -> Result<Option<User>, DomainError>;
}
