use async_graphql::SimpleObject;

pub type AuthToken = String;

pub type UserId = i64;
pub type UserName = String;
pub type UserPassword = String;
pub type UserPasswordHash = String;

#[derive(SimpleObject, Default)]
pub struct UserDTO {
    pub id: UserId,
    pub name: UserName,
    pub friends_names: Vec<UserName>,
}

impl From<FullUserDTO> for UserDTO {
    fn from(value: FullUserDTO) -> Self {
        Self {
            id: value.id,
            name: value.name,
            friends_names: value.friends_names,
        }
    }
}

pub struct FullUserDTO {
    pub id: UserId,
    pub name: UserName,
    pub password_hash: UserPasswordHash,
    pub friends_names: Vec<UserName>,
}

pub struct CreateUserDTO {
    pub name: UserName,
    pub password_hash: UserPasswordHash,
}

pub struct UserNoFriendsDTO {
    pub id: UserId,
    pub name: UserName,
    pub password_hash: UserPasswordHash,
}
