pub mod home;
pub mod auth;
pub mod not_found;

pub use home::HomePage;
pub use auth::{LoginPage, RegisterPage};
pub use not_found::NotFoundPage;