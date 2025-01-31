mod config;
mod constants;
mod database;
mod scrapper;

pub use config::*;
pub use constants::*;
pub use database::create_pool;
pub use scrapper::{get_html, get_values, parse_document};
