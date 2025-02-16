use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Error, Clone)]
pub enum ErrorCode {
  #[error("Unauthorized access")]
  Unauthorized,

  #[error("Database error: {0}")]
  Database(String),

  #[error("Validation failed: {0}")]
  Validation(String),

  #[error("Unknown error: {0}")]
  Unknown(String),
}

impl FromStr for ErrorCode {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    serde_json::from_str(s).or(Err(()))
  }
}