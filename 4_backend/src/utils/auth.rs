use crate::models::UserId;
use async_graphql::Context;
use async_trait::async_trait;

use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{RequestPartsExt, TypedHeader};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

pub fn authorize_graphql<'a>(ctx: &Context<'a>) -> Result<&'a Claims, AuthError> {
    ctx.data::<Option<Claims>>()
        .unwrap()
        .as_ref()
        .ok_or(AuthError::Unauthorized)
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub type Timestamp = i64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub user_id: UserId,
    pub exp: Timestamp,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("invalid or missing token")]
    Unauthorized,

    #[error("wrong credentials")]
    InvalidCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "invalid or missing token").into_response()
            }
            AuthError::InvalidCredentials => {
                (StatusCode::BAD_REQUEST, "wrong credentials").into_response()
            }
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::Unauthorized)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::Unauthorized)?;

        Ok(token_data.claims)
    }
}
