use super::{mutation::Mutation, schema::Query};
use crate::domain::user_repository::UserRepository;
use crate::use_case::user_use_case::UserUseCase;
use actix_web::web;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub async fn graphql<UC, UR>(
    schema: web::Data<Schema<Query<UC>, Mutation<UR>, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse
where
    UC: UserUseCase + Sync + Send,
    UR: UserRepository,
{
    schema.execute(req.into_inner()).await.into()
}
