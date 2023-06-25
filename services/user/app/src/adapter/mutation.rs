use super::schema::UserObject;
use crate::domain::user_repository::UserRepository;
use crate::use_case::mutation_use_case::MutationUseCase;
use async_graphql::Object;
use common::error::AdapterError;

pub struct Mutation<UR: UserRepository> {
    uc: MutationUseCase<UR>,
}

impl<UR: UserRepository> Mutation<UR> {
    pub fn new(uc: MutationUseCase<UR>) -> Self {
        Self { uc }
    }
}

#[Object]
impl<UR> Mutation<UR>
where
    UR: UserRepository,
{
    async fn register(
        &self,
        id_name: String,
        name: String,
        email: String,
    ) -> Result<UserObject, AdapterError> {
        let user = self.uc.register(&id_name, &name, &email).await?;
        Ok(UserObject::from(user))
    }
}
