use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct Register {
  #[cfg_attr(feature = "ssr", validate(length(min = 2, message = "First name must be at least 2 characters")))]
  pub first_name: String,

  #[cfg_attr(feature = "ssr", validate(length(min = 2, message = "Last name must be at least 2 characters")))]
  pub last_name: String,

  #[cfg_attr(feature = "ssr", validate(email(message = "Invalid email format")))]
  pub email: String,

  #[cfg_attr(feature = "ssr", validate(length(min = 8, message = "Password must be at least 8 characters")))]
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct Login {
  #[cfg_attr(feature = "ssr", validate(email(message = "Invalid email format")))]
  pub email: String,

  #[cfg_attr(feature = "ssr", validate(length(min = 8, message = "Password must be at least 8 characters")))]
  pub password: String,
}