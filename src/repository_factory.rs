use eyre::Result;
use log::LevelFilter;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use std::env;

pub fn create_pool(level_filter: LevelFilter) -> Result<PgPool> {
    let mut options: PgConnectOptions = env::var("DATABASE_URL")?.parse().unwrap();
    options = options.application_name("rustrade");
    options.log_statements(level_filter);
    let future = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options);
    Ok(async_std::task::block_on(future)?)
}
