use std::convert::Infallible;

use axum::{extract::FromRequestParts, http::request};
use axum_extra::extract::CookieJar;
use jwt_simple::{
    claims::Claims,
    prelude::{Duration, HS256Key, MACLike},
};
use password_auth::VerifyError;
use serde::{Deserialize, Serialize};

use crate::{app::AppState, error::AppError, repository::Repository};

const SECRET_KEY: &[u8] = b"im-super-secret";

pub struct GuestUser {
    username: String,
    password: String,
}

impl GuestUser {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub async fn authenticate(&self, repo: &Repository) -> Result<User, AppError> {
        let Some(record) = repo.get_user_by_name(&self.username).await? else {
            return Err(AppError::UserDoesNotExists);
        };

        match password_auth::verify_password(&self.password, &record.password_hash) {
            Ok(()) => Ok(User::new(record.id, record.username)),
            Err(VerifyError::PasswordInvalid) => Err(AppError::InvalidCredentials),
            Err(VerifyError::Parse(err)) => panic!("hashing algorithm failed: {err}"),
        }
    }

    pub async fn register(self, repo: &Repository) -> Result<User, AppError> {
        let password_hash = password_auth::generate_hash(self.password);
        let record = match repo.add_user(&self.username, &password_hash).await {
            Ok(record) => record,
            Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
                return Err(AppError::UsernameTaken);
            }
            Err(err) => {
                return Err(AppError::Database(err));
            }
        };

        Ok(User::new(record.id, record.username))
    }
}

pub struct User {
    id: i64,
    username: String,
}

impl User {
    fn new(id: i64, username: String) -> Self {
        Self { id, username }
    }

    pub const fn id(&self) -> i64 {
        self.id
    }

    pub const fn username(&self) -> &String {
        &self.username
    }

    pub fn auth_token(self) -> Result<String, AppError> {
        let key = HS256Key::from_bytes(SECRET_KEY);
        let claims = Claims::with_custom_claims(UserClaims::from(self), Duration::from_mins(10));
        let token = key.authenticate(claims)?;

        Ok(token)
    }

    pub fn from_auth_token(token: &str) -> Result<Self, AppError> {
        let key = HS256Key::from_bytes(SECRET_KEY);
        let claims = key.verify_token::<UserClaims>(token, None)?.custom;

        Ok(Self::new(claims.id, claims.username))
    }
}

impl FromRequestParts<AppState> for User {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let token = match jar.get("token") {
            Some(token) => token.value(),
            None => return Err(AppError::MissingAuthorization),
        };

        Self::from_auth_token(token)
    }
}

impl FromRequestParts<AppState> for Option<User> {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(User::from_request_parts(parts, state).await.ok())
    }
}

#[derive(Serialize, Deserialize)]
struct UserClaims {
    id: i64,
    username: String,
}

impl From<User> for UserClaims {
    fn from(User { id, username }: User) -> Self {
        Self { id, username }
    }
}
