mod config;
mod api;
mod middleware;
mod error;
mod server;

use database::Database;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ініціалізація логування
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Завантаження конфігурації
    let config = config::Config::load()?;
    
    // Підключення до БД
    let database = Database::new(&config.database_url).await?;

    // Запуск сервера
    server::run(config, database).await?;

    Ok(())
}