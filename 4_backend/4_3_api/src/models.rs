use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::{Type};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, From, Clone, Display)]
pub struct RoleSlug(pub String);

#[derive(Serialize, Deserialize, ToSchema, Debug, From, Clone, Type, Display)]
pub struct RoleName(pub String);

#[derive(Serialize, Deserialize, ToSchema, Debug, From, Clone, Display)]
pub struct RolePermissions(pub String);

#[derive(Serialize, Deserialize, ToSchema, Debug, From)]
pub struct RoleDTO {
    pub slug: RoleSlug,
    pub name: RoleName,
    pub permissions: RolePermissions,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UpdateRoleNameDTO {
    pub new_name: RoleName,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UpdateRolePermissionsDTO {
    pub new_permissions: RolePermissions,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, From, Clone, Copy, Display)]
pub struct UserId(pub i64);

#[derive(Serialize, Deserialize, ToSchema, Debug, From, Clone, Display)]
pub struct UserName(pub String);

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserDTO {
    pub id: UserId,
    pub name: UserName,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserDataDTO {
    pub name: UserName,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UpdateUserNameDTO {
    pub new_name: UserName,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct GetUserResultDTO {
    pub id: UserId,
    pub name: UserName,
    pub roles: Vec<RoleName>,
}
