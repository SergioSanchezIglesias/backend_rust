pub mod cli;
pub mod database;
#[cfg(feature = "desktop")]
pub mod desktop;
pub mod errors;
pub mod models;
pub mod repositories;

pub use errors::{AppError, Result};
