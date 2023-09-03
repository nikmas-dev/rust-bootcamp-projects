mod user;

use crate::graphql::query::user::UserQuery;
use crate::repositories::defs::user::UserRepository;
use async_graphql::MergedObject;

#[derive(MergedObject)]
pub struct Query<U: UserRepository>(UserQuery<U>);

impl<T: UserRepository> Default for Query<T> {
    fn default() -> Self {
        Self(UserQuery::default())
    }
}
