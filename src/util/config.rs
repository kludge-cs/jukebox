extern crate config;
extern crate serde;

use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::process::exit;

#[derive(Debug, Deserialize, Serialize)]
struct Server {
	port: u16,
	address: Box<str>,
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
#[allow(non_snake_case)]
struct JukeboxRatelimit {
	ipBlocks: Option<Vec<Box<str>>>,
	excludedIps: Option<Vec<Box<str>>>,
	strategy: Box<str>,
	searchTriggersFail: bool,
	retryLimit: i8,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
struct JukeboxServer {
	password: Option<Box<str>>,
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
	root: Box<str>,
	lavalink: Box<str>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Logging {
	path: Option<Box<str>>,
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
		let mut conf = Config::default();

		conf.merge(Config::try_from(&Settings::default())?)?;

		let srcs = vec![
			File::new("application", FileFormat::Yaml).required(false),
			File::new("jukebox", FileFormat::Toml).required(false),
		];
		for f in srcs {
			if let Err(ConfigError::FileParse { uri, cause }) = conf.merge(f) {
				eprintln!("Error parsing {}: {}", uri.unwrap(), cause);
				exit(115);
			}
		}

		conf.merge(Environment::with_prefix("jukebox"))?;
		conf.try_into()
	}
}

impl Default for Settings {
	fn default() -> Self {
		Settings {
			server: Server {
				port: 2333,
				address: "0.0.0.0".to_owned().into_boxed_str(),
			},
			lavalink: Jukebox {
				server: JukeboxServer {
					password: Some(
						"youshallnotpass".to_owned().into_boxed_str(),
					),
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
						strategy: "RotateOnBan".to_owned().into_boxed_str(),
						searchTriggersFail: true,
						retryLimit: -1,
					},
				},
			},
			logging: Logging {
				path: None,
				level: LoggingLevel {
					root: "INFO".to_owned().into_boxed_str(),
					lavalink: "INFO".to_owned().into_boxed_str(),
				},
			},
		}
	}
}
