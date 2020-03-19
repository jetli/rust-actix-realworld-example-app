use anyhow::Result;
use async_std::task;
use async_trait::async_trait;

use conduit_primitives::users::errors::UserError;
use conduit_primitives::users::repos::UsersRepo as UsersRepoTrait;
use conduit_primitives::users::types::User;

pub struct UsersRepo {
    pool: sqlx::Pool<sqlx::MySqlConnection>,
}

impl UsersRepo {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
        let pool = task::block_on(sqlx::MySqlPool::new(database_url.as_str()))
            .expect("Failed to create pool");
        Self { pool }
    }
}

#[async_trait]
impl UsersRepoTrait for UsersRepo {
    async fn get_by_id(&self, user_id: u64) -> Result<User, UserError> {
        let row = sqlx::query!(
            "SELECT id, username FROM conduit_users WHERE id = ?",
            user_id
        )
        .fetch_one(&mut &self.pool)
        .await;

        if let Ok(row) = row {
            Ok(User {
                id: row.id,
                username: row.username,
            })
        } else {
            Err(UserError::DoesNotExists)
        }
    }
}
