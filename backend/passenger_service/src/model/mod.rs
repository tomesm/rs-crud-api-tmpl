use thiserror::Error as ThisError;

mod db;
mod passenger;

// re-export to the outside world
pub use db::init_db;
pub use db::Db;
pub use passenger::{Passenger, PassengerDao, PassengerPatch};

// region:    Error
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}[{1}] ")]
    EntityNotFound(&'static str, String),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

// endregion: Error
