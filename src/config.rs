use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
	servers: Vec<Server>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
	pub name: String,
	pub ip: String,
	pub port: u16,
}

impl Server {
	pub fn new(name: &str, ip: &str, port: u16) -> Self {
		Self {
			name: name.to_owned(),
			ip: ip.to_owned(),
			port,
		}
	}
}

pub fn load_config() -> Result<Vec<Server>> {
	static POSSIBLE_CONFIG_FILES: &[&str] = &[
		"/ss13.toml",
		"/ss13-status.toml",
		"/config/ss13.toml",
		"/config/ss13-status.toml",
		"/3ds/ss13.toml",
		"/3ds/ss13-status.toml",
		"/3ds/ss13-status/config.toml",
	];

	let mut config_path: Option<&str> = None;
	for path in POSSIBLE_CONFIG_FILES {
		if std::fs::exists(path).unwrap_or(false) {
			config_path = Some(*path);
			break;
		}
	}
	let config_path = match config_path {
		Some(path) => path,
		None => return Ok(default_servers()),
	};

	let config_file = std::fs::read(config_path)
		.with_context(|| format!("failed to read config file at {config_path}"))?;

	toml::from_slice::<Config>(&config_file)
		.with_context(|| format!("failed to parse config file at {config_path}"))
		.map(|config| config.servers)
}

pub fn default_servers() -> Vec<Server> {
	vec![
		Server::new("Monkestation MRP1", "104.194.9.21", 3121),
		Server::new("Monkestation MRP2", "104.194.9.21", 3122),
		Server::new("Oculis", "104.194.9.21", 42069),
	]
}
