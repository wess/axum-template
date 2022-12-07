use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub url: String,
  pub auto_migrate: bool,
  pub max_connections: u32,
  pub min_connections: u32,
  pub max_lifetime: u64,
  pub connect_timeout: u64,
  pub idle_timeout: u64,
  pub migrations_folder: String,
}

impl Config {
  pub fn with(url:&str) -> Self {
    Self::new(
      url,
      None,
      None,
      None,
      None,
      None,
      None,
      None
    )
  }

  pub fn new(
    url: &str,
    auto_migrate: Option<bool>,
    max_connections: Option<u32>,
    min_connections: Option<u32>,
    max_lifetime: Option<u64>,
    connect_timeout: Option<u64>,
    idle_timeout: Option<u64>,
    migrations_folder: Option<String>
  ) -> Self {
    Config {
      url: url.to_string(),
      auto_migrate: auto_migrate.unwrap_or(true),
      max_connections: max_connections.unwrap_or(5),
      min_connections: min_connections.unwrap_or(1),
      max_lifetime: max_lifetime.unwrap_or(300),
      connect_timeout: connect_timeout.unwrap_or(5),
      idle_timeout: idle_timeout.unwrap_or(300),
      migrations_folder: migrations_folder.unwrap_or(string!("./migrations")),
    }
  }
}