use crate::{models::UserRecord, repository::Repository};

impl Repository {
    pub async fn add_user(&self, username: &str, password_hash: &str) -> sqlx::Result<UserRecord> {
        sqlx::query_as!(
            UserRecord,
            "INSERT INTO users (username, password)
             VALUES ($1, $2)
             RETURNING id, username, password AS password_hash;
            ",
            username,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_user_by_name(&self, username: &str) -> sqlx::Result<Option<UserRecord>> {
        sqlx::query_as!(
            UserRecord,
            "SELECT id, username, password AS password_hash
             FROM users
             WHERE username = $1;",
            username,
        )
        .fetch_optional(&self.pool)
        .await
    }
}
