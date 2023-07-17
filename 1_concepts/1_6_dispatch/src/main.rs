use std::borrow::Cow;
use thiserror::Error;

pub type UserId = u64;

#[derive(Error, Debug)]
#[error("the user with id {0} was not found")]
pub struct UserNotFoundError(UserId);

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    email: Cow<'static, str>,
    activated: bool,
}

pub struct StaticUserRepository<T: Storage<UserId, User>>(T);

impl<T: Storage<UserId, User>> StaticUserRepository<T> {
    pub fn new(storage: T) -> Self {
        Self(storage)
    }

    pub fn get(&self, id: UserId) -> Option<&User> {
        self.0.get(&id)
    }

    pub fn add(&mut self, user: User) {
        self.0.set(user.id, user)
    }

    pub fn update(&mut self, user: User) -> Result<(), UserNotFoundError> {
        if self.0.get(&user.id).is_none() {
            return Err(UserNotFoundError(user.id));
        }

        self.0.set(user.id, user);
        Ok(())
    }

    pub fn remove(&mut self, id: UserId) -> Option<User> {
        self.0.remove(&id)
    }
}

pub struct DynamicUserRepository(Box<dyn Storage<UserId, User>>);

impl DynamicUserRepository {
    pub fn new(storage: Box<dyn Storage<UserId, User>>) -> Self {
        Self(storage)
    }

    pub fn get(&self, id: UserId) -> Option<&User> {
        self.0.get(&id)
    }

    pub fn add(&mut self, user: User) {
        self.0.set(user.id, user)
    }

    pub fn update(&mut self, user: User) -> Result<(), UserNotFoundError> {
        if self.0.get(&user.id).is_none() {
            return Err(UserNotFoundError(user.id));
        }

        self.0.set(user.id, user);
        Ok(())
    }

    pub fn remove(&mut self, id: UserId) -> Option<User> {
        self.0.remove(&id)
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    struct TestStorage(HashMap<UserId, User>);

    impl Storage<UserId, User> for TestStorage {
        fn set(&mut self, key: UserId, val: User) {
            self.0.insert(key, val);
        }

        fn get(&self, key: &UserId) -> Option<&User> {
            self.0.get(key)
        }

        fn remove(&mut self, key: &UserId) -> Option<User> {
            self.0.remove(key)
        }
    }

    #[test]
    fn should_successfully_add_and_get_user() {
        let storage = TestStorage(HashMap::new());
        let mut static_repository = StaticUserRepository::new(storage.clone());
        let mut dyn_repository = DynamicUserRepository::new(Box::new(storage));

        let user = User {
            id: 1,
            email: Cow::Borrowed("nikmas@gmail.com"),
            activated: true,
        };

        static_repository.add(user.clone());
        static_repository.get(1).unwrap();

        dyn_repository.add(user);
        dyn_repository.get(1).unwrap();
    }

    #[test]
    fn should_successfully_update_user() {
        let storage = TestStorage(HashMap::new());
        let mut static_repository = StaticUserRepository::new(storage.clone());
        let mut dyn_repository = DynamicUserRepository::new(Box::new(storage));

        let user = User {
            id: 1,
            email: Cow::Borrowed("nikmas@gmail.com"),
            activated: true,
        };

        static_repository.add(user.clone());
        dyn_repository.add(user);

        assert_eq!(static_repository.get(1).unwrap().email, "nikmas@gmail.com");
        assert_eq!(dyn_repository.get(1).unwrap().email, "nikmas@gmail.com");

        let new_user = User {
            id: 1,
            email: Cow::Borrowed("hello@gmail.com"),
            activated: true,
        };

        static_repository.update(new_user.clone()).unwrap();
        dyn_repository.update(new_user).unwrap();

        assert_eq!(static_repository.get(1).unwrap().email, "hello@gmail.com");
        assert_eq!(dyn_repository.get(1).unwrap().email, "hello@gmail.com");
    }

    #[test]
    fn should_fail_to_update_nonexistent_user() {
        let storage = TestStorage(HashMap::new());
        let mut static_repository = StaticUserRepository::new(storage.clone());
        let mut dyn_repository = DynamicUserRepository::new(Box::new(storage));

        let user = User {
            id: 1,
            email: Cow::Borrowed("nikmas@gmail.com"),
            activated: true,
        };

        static_repository.update(user.clone()).unwrap_err();
        dyn_repository.update(user).unwrap_err();
    }

    #[test]
    fn should_successfully_remove_user() {
        let storage = TestStorage(HashMap::new());
        let mut static_repository = StaticUserRepository::new(storage.clone());
        let mut dyn_repository = DynamicUserRepository::new(Box::new(storage));

        let user = User {
            id: 1,
            email: Cow::Borrowed("nikmas@gmail.com"),
            activated: true,
        };

        static_repository.add(user.clone());
        dyn_repository.add(user);

        static_repository.get(1).unwrap();
        dyn_repository.get(1).unwrap();

        static_repository.remove(1);
        dyn_repository.remove(1);

        assert!(static_repository.get(1).is_none());
        assert!(dyn_repository.get(1).is_none());
    }
}
