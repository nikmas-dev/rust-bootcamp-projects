use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type RoleSlug = String;
pub type RoleName = String;
pub type RolePermissions = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleDTO {
    pub slug: RoleSlug,
    pub name: RoleName,
    pub permissions: RolePermissions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRoleNameDTO {
    pub new_name: RoleName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRolePermissionsDTO {
    pub new_permissions: RolePermissions,
}

pub type UserId = i64;
pub type UserName = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDTO {
    pub id: UserId,
    pub name: UserName,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserDataDTO {
    pub name: UserName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserNameDTO {
    pub new_name: UserName,
}

pub type AllUserRoles = Vec<RoleName>;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct GetUserResultDTO {
    pub id: UserId,
    pub name: UserName,
    pub roles: AllUserRoles,
}
