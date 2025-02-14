use std::env;

use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use leptos_image::ImageOptimizer;
use sea_orm::{prelude::*, Database};

#[derive(Clone, FromRef, Debug)]
pub struct AppState {
  pub leptos_options: LeptosOptions,
  pub optimizer: ImageOptimizer,
  pub db: DatabaseConnection,
}

pub async fn init_state(leptos_options: LeptosOptions) -> Result<AppState, ()> 
{
   // Connect to db
   let db: DatabaseConnection = Database::connect(
        env::var("DATABASE_URL").expect("$DATABASE_URL is not set")
    ).await.unwrap();


  // Optimizer
  let optimizer = ImageOptimizer::new("/__cache/image", leptos_options.site_root.to_string(), 1);

  // State
  Ok(AppState {
      leptos_options,
      optimizer,
      db,
  })
}