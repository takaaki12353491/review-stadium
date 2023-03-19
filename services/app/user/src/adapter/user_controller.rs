use super::schema::{Mutation, Query};
use crate::use_case::user_use_case::UserUseCase;
use actix_web::web;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub async fn graphql<UC>(
    schema: web::Data<Schema<Query<UC>, Mutation<UC>, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse
where
    UC: UserUseCase + Sync + Send,
{
    schema.execute(req.into_inner()).await.into()
}
