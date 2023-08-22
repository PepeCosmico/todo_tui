use thiserror::Error as ThisError;

use crate::db::DbError;

#[derive(ThisError, Debug, PartialEq, Eq)]
pub enum AppError {
    #[error(transparent)]
    DbError(#[from] DbError),
}
