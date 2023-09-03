mod user;

use crate::graphql::mutation::user::UserMutation;
use crate::repositories::defs::user::UserRepository;
use async_graphql::MergedObject;

#[derive(MergedObject)]
pub struct Mutation<U: UserRepository>(UserMutation<U>);

impl<T: UserRepository> Default for Mutation<T> {
    fn default() -> Self {
        Self(UserMutation::default())
    }
}
