pub use ::config::ConfigError;
use serde::Deserialize;
use deadpool_postgres::{RecyclingMethod, ManagerConfig};

#[derive(Deserialize)]
pub struct Config {
	pub workers: usize,
	pub pg: deadpool_postgres::Config,
}

impl Config {
	pub fn from_env() -> Result<Self, ConfigError> {
		let mut cfg = ::config::Config::new();
		cfg.merge(::config::Environment::new())?;
		let mut cfg: Config = cfg.try_into()?;
		cfg.pg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
		Ok(cfg)
	}
}
