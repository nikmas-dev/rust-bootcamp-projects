use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("the user with id {0} was not found")]
pub struct UserNotFoundError(UserId);

pub type UserId = u64;

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    email: Cow<'static, str>,
    activated: bool,
}

pub trait UserRepository {
    fn get(&self, id: UserId) -> Option<&User>;
    fn add(&mut self, user: User);
    fn update(&mut self, user: User) -> Result<(), UserNotFoundError>;
    fn remove(&mut self, id: UserId) -> Option<User>;
}

pub trait Command {
    type Context: ?Sized;
    type Result;

    fn execute(&self, user: &User, ctx: &mut Self::Context) -> Self::Result;
}

pub struct CreateUser;

impl Command for CreateUser {
    type Context = dyn UserRepository;
    type Result = ();

    fn execute(&self, user: &User, ctx: &mut Self::Context) -> Self::Result {
        ctx.add(user.clone())
    }
}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = ();

    fn handle_command(&self, cmd: &CreateUser, ctx: &mut Self::Context) -> Self::Result {
        cmd.execute(self, ctx)
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    impl UserRepository for HashMap<UserId, User> {
        fn get(&self, id: UserId) -> Option<&User> {
            self.get(&id)
        }

        fn add(&mut self, user: User) {
            self.insert(user.id, user);
        }

        fn update(&mut self, user: User) -> Result<(), UserNotFoundError> {
            if self.get(&user.id).is_none() {
                return Err(UserNotFoundError(user.id));
            }

            self.insert(user.id, user);

            Ok(())
        }

        fn remove(&mut self, id: UserId) -> Option<User> {
            self.remove(&id)
        }
    }

    #[test]
    fn should_successfully_create_user_through_command_handler() {
        let mut repository = HashMap::new();

        let user = User {
            id: 1,
            email: Cow::Borrowed("nikmas@gmail.com"),
            activated: true,
        };

        let create_user_command = CreateUser;

        user.handle_command(&create_user_command, &mut repository);

        repository.get(&user.id).unwrap();
    }
}
