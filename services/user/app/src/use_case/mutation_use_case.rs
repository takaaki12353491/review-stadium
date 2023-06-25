use crate::domain::{user::User, user_repository::UserRepository};
use async_trait::async_trait;
use common::error::UseCaseError;

#[async_trait]
pub trait MutationUseCase: Send + Sync + 'static {
    async fn register(&self, id_name: &str, name: &str, email: &str) -> Result<User, UseCaseError>;
}

#[derive(Debug, Clone)]
pub struct MutationInteractor<UR: UserRepository> {
    user_repository: UR,
}

impl<UR: UserRepository> MutationInteractor<UR> {
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<UR> MutationUseCase for MutationInteractor<UR>
where
    UR: UserRepository,
{
    async fn register(&self, id_name: &str, name: &str, email: &str) -> Result<User, UseCaseError> {
        let user = self.user_repository.create(&id_name, &name, &email).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_repository::MockUserRepository;
    use anyhow::Error;
    use common::error::DomainError;
    use mockall::predicate::always;

    #[tokio::test]
    async fn test_register_success() {
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_create()
            .times(1)
            .with(always(), always(), always())
            .returning(|_, _, _| {
                User::new(
                    String::from("id_name"),
                    String::from("name"),
                    String::from("sample@example.com"),
                )
            });

        let user_usecase = MutationInteractor::new(mock_user_repository);
        let res = user_usecase
            .register(
                &String::from("id_name"),
                &String::from("name"),
                &String::from("sample@example.com"),
            )
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_register_failed() {
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_create()
            .times(1)
            .with(always(), always(), always())
            .returning(|_, _, _| Err(DomainError::RepositoryError(Error::msg("Database Error"))));

        let user_usecase = MutationInteractor::new(mock_user_repository);
        let res = user_usecase
            .register(
                &String::from("id_name"),
                &String::from("name"),
                &String::from("sample@example.com"),
            )
            .await;

        assert!(res.is_err());
    }
}
