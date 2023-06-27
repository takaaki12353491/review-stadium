use super::user::User;
use async_trait::async_trait;
use common::{error::DomainError, model::ID};
use mockall::automock;
use std::fmt::Debug;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync + Debug + 'static {
    async fn create(&self, id_name: &str, name: &str, email: &str) -> Result<User, DomainError>;
    async fn find_by_id(&self, id: &ID) -> Result<Option<User>, DomainError>;
    async fn find_by_id_name(&self, id_name: &str) -> Result<Option<User>, DomainError>;
}
