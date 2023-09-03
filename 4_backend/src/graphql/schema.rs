use crate::graphql::mutation::Mutation;
use crate::graphql::query::Query;
use crate::repositories::defs::user::UserRepository;
use async_graphql::{EmptySubscription, Schema};

pub type AppSchema<U> = Schema<Query<U>, Mutation<U>, EmptySubscription>;

pub fn build_schema<U: UserRepository>() -> AppSchema<U> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish()
}
