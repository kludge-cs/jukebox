extern crate config;
extern crate serde;

use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Server {
	port: u16,
	address: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JukeboxSources {
	youtube: bool,
	bandcamp: bool,
	soundcloud: bool,
	twitch: bool,
	vimeo: bool,
	http: bool,
	local: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct JukeboxRatelimit {
	ipBlocks: Option<Vec<String>>,
	excludedIps: Option<Vec<String>>,
	strategy: String,
	searchTriggersFail: bool,
	retryLimit: i8,
}

#[derive(Debug, Deserialize, Serialize)]
struct JukeboxServer {
	password: Option<String>,
	sources: JukeboxSources,
	bufferDurationMs: u32,
	youtubePlaylistLoadLimit: u8,
	playerUpdateInterval: u8,
	youtubeSearchEnabled: bool,
	soundcloudSearchEnabled: bool,
	ratelimit: JukeboxRatelimit,
}

#[derive(Debug, Deserialize, Serialize)]
struct Jukebox {
	server: JukeboxServer,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggingLevel {
	root: String,
	lavalink: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Logging {
	path: Option<String>,
	level: LoggingLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
	server: Server,
	lavalink: Jukebox,
	logging: Logging,
}

impl Settings {
	pub fn new() -> Result<Self, ConfigError> {
		let mut settings = Config::default();
		settings
			.merge(Config::try_from(&Self::default())?)?
			.merge(File::new("application", FileFormat::Yaml).required(false))?
			.merge(File::new("jukebox", FileFormat::Toml).required(false))?
			.merge(Environment::with_prefix("jukebox"))?;
		settings.try_into()
	}
}

impl Default for Settings {
	fn default() -> Self {
		Settings {
			server: Server {
				port: 2333,
				address: "0.0.0.0".to_string(),
			},
			lavalink: Jukebox {
				server: JukeboxServer {
					password: Some("youshallnotpass".to_string()),
					sources: JukeboxSources {
						youtube: true,
						bandcamp: true,
						soundcloud: true,
						twitch: true,
						vimeo: true,
						http: true,
						local: false,
					},
					bufferDurationMs: 400,
					youtubePlaylistLoadLimit: 6,
					playerUpdateInterval: 5,
					youtubeSearchEnabled: true,
					soundcloudSearchEnabled: true,
					ratelimit: JukeboxRatelimit {
						ipBlocks: None,
						excludedIps: None,
						strategy: "RotateOnBan".to_string(),
						searchTriggersFail: true,
						retryLimit: -1,
					},
				},
			},
			logging: Logging {
				path: None,
				level: LoggingLevel {
					root: "INFO".to_string(),
					lavalink: "INFO".to_string(),
				},
			},
		}
	}
}
