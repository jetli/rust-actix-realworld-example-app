use anyhow::Result;
use async_std::task;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use conduit_primitives::users::errors::UserError;
use conduit_primitives::users::repos::UsersRepo as UsersRepoTrait;
use conduit_primitives::users::types::User;

pub struct UsersRepo {
    pool: r2d2::Pool<ConnectionManager<MysqlConnection>>,
}

impl UsersRepo {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        Self { pool }
    }
}

#[async_trait]
impl UsersRepoTrait for UsersRepo {
    async fn get_by_id(&self, user_id: u64) -> Result<User, UserError> {
        use crate::schema::conduit_users::dsl::*;

        let conn = self
            .pool
            .get()
            .expect("Failed to get a connction from pool");

        let user = task::spawn_blocking(move || -> QueryResult<super::types::User> {
            let user = conduit_users
                .filter(id.eq(user_id))
                .first::<super::types::User>(&conn);

            user
        })
        .await;

        if let Ok(user) = user {
            Ok(User {
                id: user.id,
                username: user.username,
            })
        } else {
            Err(UserError::DoesNotExists)
        }
    }
}
