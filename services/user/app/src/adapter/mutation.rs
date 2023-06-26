use super::schema::UserObject;
use crate::use_case::mutation_use_case::MutationUseCase;
use async_graphql::Object;
use common::error::AdapterError;

pub struct Mutation<MUC: MutationUseCase> {
    uc: MUC,
}

impl<MUC: MutationUseCase> Mutation<MUC> {
    pub fn new(uc: MUC) -> Self {
        Self { uc }
    }
}

#[Object]
impl<MUC> Mutation<MUC>
where
    MUC: MutationUseCase,
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
