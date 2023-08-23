use im::HashMap;

trait UserRepository {
    type UserId;
    type User;

    fn find_one_by_id(&self, id: &Self::UserId) -> Option<Self::User>;
    fn find_many_by_ids(&self, ids: &[Self::UserId]) -> Vec<Self::User>;
    fn find_users_with_name_containing(&self, string: &str) -> Vec<Self::UserId>;
}

type UserId = u64;
type UserName = String;

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: UserId,
    name: UserName,
}

#[derive(Debug)]
struct InMemoryUserRepository {
    users: HashMap<UserId, User>,
}

impl UserRepository for InMemoryUserRepository {
    type UserId = UserId;
    type User = User;

    fn find_one_by_id(&self, id: &Self::UserId) -> Option<Self::User> {
        self.users.get(id).cloned()
    }

    fn find_many_by_ids(&self, ids: &[Self::UserId]) -> Vec<Self::User> {
        let mut found_users = Vec::new();

        for id in ids {
            if let Some(user) = self.find_one_by_id(id) {
                found_users.push(user);
            }
        }

        found_users
    }

    fn find_users_with_name_containing(&self, string: &str) -> Vec<Self::UserId> {
        let mut found_users = Vec::new();

        for user in self.users.values() {
            if user.name.contains(string) {
                found_users.push(user.id);
            }
        }

        found_users
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use super::*;

    mod find_one_by_id {
        use super::*;
        use im::hashmap;

        #[test]
        fn returns_some_user_when_id_exists() {
            let user = User {
                id: 1,
                name: "John".to_string(),
            };

            let repository = InMemoryUserRepository {
                users: hashmap! {
                    user.id => user.clone(),
                },
            };

            assert_eq!(repository.find_one_by_id(&user.id), Some(user));
        }

        #[test]
        fn returns_none_when_id_does_not_exist() {
            let repository = InMemoryUserRepository { users: hashmap! {} };

            assert_eq!(repository.find_one_by_id(&1), None);
        }
    }

    mod find_many_by_ids {
        use super::*;
        use im::hashmap;

        #[test]
        fn returns_users_when_ids_exist() {
            let user1 = User {
                id: 1,
                name: "John".to_string(),
            };

            let user2 = User {
                id: 2,
                name: "Jane".to_string(),
            };

            let repository = InMemoryUserRepository {
                users: hashmap! {
                    user1.id => user1.clone(),
                    user2.id => user2.clone(),
                },
            };

            assert_eq!(
                repository.find_many_by_ids(&[user1.id, user2.id]),
                vec![user1, user2]
            );
        }

        #[test]
        fn returns_some_users_if_some_ids_exist() {
            let user1 = User {
                id: 1,
                name: "John".to_string(),
            };

            let repository = InMemoryUserRepository {
                users: hashmap! {
                    user1.id => user1.clone(),
                },
            };

            assert_eq!(repository.find_many_by_ids(&[user1.id, 2]), vec![user1]);
        }

        #[test]
        fn returns_empty_vec_when_ids_do_not_exist() {
            let repository = InMemoryUserRepository { users: hashmap! {} };

            assert_eq!(repository.find_many_by_ids(&[1, 2]), vec![]);
        }
    }

    mod find_users_with_name_containing {
        use super::*;
        use im::hashmap;

        #[test]
        fn returns_users_when_name_contains_string() {
            let user1 = User {
                id: 1,
                name: "John".to_string(),
            };

            let user2 = User {
                id: 2,
                name: "Jane".to_string(),
            };

            let repository = InMemoryUserRepository {
                users: hashmap! {
                    user1.id => user1,
                    user2.id => user2.clone(),
                },
            };

            assert_eq!(
                repository.find_users_with_name_containing("a"),
                vec![user2.id]
            );
        }
    }
}
