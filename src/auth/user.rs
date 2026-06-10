use password_auth::VerifyError;

use crate::{error::AppError, repository::Repository};

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
}
