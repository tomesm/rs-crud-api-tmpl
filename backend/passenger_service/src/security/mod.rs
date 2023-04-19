use crate::model::Db;
use thiserror::Error as ThisError;

#[derive(Debug)]
pub struct UserCtx {
    pub user_id: String,
}

pub async fn utx_from_token(_db: &Db, token: &str) -> Result<UserCtx, Error> {
    // todo!("Real validation needed");
    // for now, just parse to i64
    match token.parse::<String>() {
        Ok(user_id) => Ok(UserCtx { user_id }),
        Err(_) => Err(Error::InvalidToken(token.to_string())),
    }
}

// region:    Error
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Invalid Token {0}")]
    InvalidToken(String),
}

// endregion: Error
