use std::fmt;

#[derive(Debug)]
pub enum RepositoryError {
    DatabaseError(rusqlite::Error),
    LockError(String),
    NotFound(String),
    InvalidData(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            RepositoryError::LockError(msg) => write!(f, "Lock error: {}", msg),
            RepositoryError::NotFound(msg) => write!(f, "Not found: {}", msg),
            RepositoryError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<rusqlite::Error> for RepositoryError {
    fn from(err: rusqlite::Error) -> Self {
        RepositoryError::DatabaseError(err)
    }
}

impl From<RepositoryError> for String {
    fn from(err: RepositoryError) -> Self {
        err.to_string()
    }
}

pub type Result<T> = std::result::Result<T, RepositoryError>;
