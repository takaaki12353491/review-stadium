use super::{mutation::Mutation, schema::Query};
use crate::use_case::{mutation_use_case::MutationUseCase, user_use_case::UserUseCase};
use actix_web::web;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub async fn graphql<UC, MUC>(
    schema: web::Data<Schema<Query<UC>, Mutation<MUC>, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse
where
    UC: UserUseCase + Sync + Send,
    MUC: MutationUseCase + Send + Sync,
{
    schema.execute(req.into_inner()).await.into()
}
