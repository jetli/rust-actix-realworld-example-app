use anyhow::Result;
use async_trait::async_trait;

use conduit_primitives::users::repos::UsersRepo as UsersRepoTrait;
use conduit_primitives::users::types::User;

pub struct UsersRepo {}

#[async_trait]
impl UsersRepoTrait for UsersRepo {
    async fn get_by_id(&self, id: u64) -> Result<User> {
        // TO DO: Async access of diesel
        Ok(User { id })
    }
}
