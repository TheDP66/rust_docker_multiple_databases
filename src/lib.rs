use sqlx::{Pool, Postgres};
use utils::config::Config;

pub mod dtos;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_codein: Pool<Postgres>,
    pub db_iekimtie: Pool<Postgres>,
    pub db_wkawan: Pool<Postgres>,
    pub config: Config,
}
