use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
  #[error("Query error: {0}")]
  Query(String),

  #[error("Connection error: {0}")]
  Connection(String),

  #[error("Migration error: {0}")]
  Migration(String),

  #[error("Unknown database error")]
  Unknown
}