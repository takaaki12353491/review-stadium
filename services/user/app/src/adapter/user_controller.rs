use super::{mutation::Mutation, query::Query};
use crate::domain::user_repository::UserRepository;
use actix_web::web;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub async fn graphql<UR>(
    schema: web::Data<Schema<Query<UR>, Mutation<UR>, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse
where
    UR: UserRepository + Send + Sync + 'static,
{
    schema.execute(req.into_inner()).await.into()
}
