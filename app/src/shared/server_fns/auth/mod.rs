pub mod dto;

use dto::*;

use super::errors::ErrorCode;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use {
  sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait},
  validator::Validate,
  argon2::{password_hash::{rand_core::OsRng, SaltString, PasswordHasher}, Argon2},
  crate::shared::database::entries::*,
  crate::shared::state::AppState,
};

#[server(RegisterUser)]
pub async fn register_user(request: Register) -> Result<(), ServerFnError<ErrorCode>> {
  // Валідація вхідних даних
  if let Err(errors) = request.validate() {
    return Err(ServerFnError::WrappedServerError(ErrorCode::Validation(format!("{:?}", errors))));
  }

  // Отримання підключення до бази даних
  let app_state = use_context::<AppState>()
    .ok_or_else(|| ServerFnError::ServerError("AppState not found".to_string()))?;

  let db = &app_state.db;

  // Перевірка, чи існує користувач із таким email
  let existing_user = users::Entity::find()
    .filter(users::Column::Email.eq(request.email.clone()))
    .one(db)
    .await
    .map_err(|_| ServerFnError::ServerError("Database query failed".into()))?;

  if existing_user.is_some() {
    return Err(ServerFnError::WrappedServerError(
      ErrorCode::Validation("User already exists".to_string()),
    ));
  }

  // Хешування паролю
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let hash = argon2.hash_password(request.password.as_bytes(), &salt).unwrap().to_string();

  // Створення нового користувача
  let new_user = users::ActiveModel {
    first_name: Set(request.first_name),
    last_name: Set(request.last_name),
    email: Set(request.email),
    password_hash: Set(hash),
    ..Default::default()
  };

  new_user.insert(db).await.map_err(|_| {
    ServerFnError::ServerError("Failed to insert user into database".into())
  })?;

  Ok(())
}
