#[derive(thiserror::Error, Clone, Debug)]
pub enum UserError {
    #[error("User does not exists")]
    DoesNotExists,
}
