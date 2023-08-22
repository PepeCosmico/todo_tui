use thiserror::Error as ThisError;

pub mod database;

pub use self::database::Db;

#[derive(ThisError, Debug, PartialEq, Eq)]
pub enum DbError {
    #[error("Db file not found: {0}")]
    DbNotFound(&'static str),
    #[error("Db file has bad format")]
    DbBadFormat,
}

#[cfg(test)]
#[path = "../../_tests/db_tests/mod.rs"]
mod tests;
