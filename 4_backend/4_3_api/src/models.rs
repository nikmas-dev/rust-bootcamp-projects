use serde::Serialize;

pub type RoleSlug = String;
pub type RoleName = String;
pub type RolePermissions = String;

#[derive(Serialize, Debug)]
pub struct Role {
    pub slug: RoleSlug,
    pub name: RoleName,
    pub permissions: RolePermissions,
}

pub type UserId = i64;
pub type UserName = String;

#[derive(Serialize, Debug)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
}

#[derive(Debug)]
pub struct UserData {
    pub name: UserName,
}

pub type AllUserRoles = Vec<RoleName>;

#[derive(Serialize, Debug)]
pub struct GetUserResult {
    pub id: UserId,
    pub name: UserName,
    pub roles: AllUserRoles,
}
