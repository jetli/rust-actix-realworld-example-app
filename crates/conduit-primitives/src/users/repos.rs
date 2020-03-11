use anyhow::Result;
use async_trait::async_trait;

use super::types::User;

#[async_trait]
pub trait UsersRepo {
    async fn get_by_id(&self, id: u64) -> Result<User>;
}
