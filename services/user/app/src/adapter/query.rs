use super::schema::UserObject;
use crate::domain::user_repository::UserRepository;
use crate::use_case::query_use_case::QueryUseCase;
use async_graphql::Object;
use common::error::AdapterError;

pub struct Query<UR: UserRepository> {
    uc: QueryUseCase<UR>,
}

impl<UR: UserRepository> Query<UR> {
    pub fn new(uc: QueryUseCase<UR>) -> Self {
        Self { uc }
    }
}

#[Object]
impl<UR> Query<UR>
where
    UR: UserRepository,
{
    async fn get_by_id_name(&self, id_name: String) -> Result<Option<UserObject>, AdapterError> {
        let user = self.uc.get_by_id_name(&id_name).await?;
        Ok(user.map(|user| user.into()))
    }
}
