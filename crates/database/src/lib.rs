use std::{
  time::Duration,
  path::Path
};

use sqlx::postgres::PgPoolOptions;

#[macro_use]
extern crate oxide;

mod error;
pub use error::DatabaseError;

mod result;
pub use result::DatabaseResult;

mod config;
pub use config::Config;

mod value;
pub use value::Value;

pub type DBPool = sqlx::Pool<sqlx::Postgres>;

pub async fn init(config: &Config) -> crate::DatabaseResult<DBPool> {
  let conn = build(config).await.unwrap();

  if config.auto_migrate {
    migrate(&config, None).await?;
  }

  Ok(
    conn
  )
}

pub async fn migrate(config: &Config, folder_path:Option<String>) -> crate::DatabaseResult<()> {
    console_info!("Running database migrations...");

    let m_path = match folder_path {
      Some(m) => m,
      None => config.migrations_folder.clone() 
    };

    let conn = build(config).await.unwrap();

    sqlx::migrate::Migrator::new(Path::new(&m_path))
    .await
    .unwrap()
    .run(&conn)
    .await
    .unwrap();

    conn.close().await;

  Ok(())
}  

async fn build(config: &Config) -> crate::DatabaseResult<DBPool> {

  let url = &config.url;
  let max_connections = config.max_connections;
  let min_connections = config.min_connections;
  let max_lifetime = config.max_lifetime;
  let connect_timeout = config.connect_timeout;
  let idle_timeout = config.idle_timeout;

  PgPoolOptions::new()
    .max_connections(max_connections)
    .min_connections(min_connections)
    .max_lifetime(Some(Duration::from_secs(max_lifetime)))
    .acquire_timeout(Duration::from_secs(connect_timeout))
    .idle_timeout(Duration::from_secs(idle_timeout))
    .test_before_acquire(true)
    .connect(url)
    .await
    .map_err(|err| DatabaseError::Connection(err.to_string()))

}