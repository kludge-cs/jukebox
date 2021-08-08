extern crate config;
extern crate serde;

pub use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Server {
	port: u16,
	address: Box<str>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
struct JukeboxRatelimit {
	ip_blocks: Option<Vec<Box<str>>>,
	excluded_ips: Option<Vec<Box<str>>>,
	strategy: Box<str>,
	search_triggers_fail: bool,
	retry_limit: i8,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct JukeboxServer {
	password: Option<Box<str>>,
	sources: JukeboxSources,
	buffer_duration_ms: u32,
	youtube_playlist_load_limit: u8,
	player_update_interval: u8,
	youtube_search_enabled: bool,
	soundcloud_search_enabled: bool,
	ratelimit: JukeboxRatelimit,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Jukebox {
	server: JukeboxServer,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoggingLevel {
	root: Box<str>,
	lavalink: Box<str>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Logging {
	path: Option<Box<str>>,
	level: LoggingLevel,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
			// TODO: establish why this fails silently instead of erroring
			conf.merge(f)?;
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
					buffer_duration_ms: 400,
					youtube_playlist_load_limit: 6,
					player_update_interval: 5,
					youtube_search_enabled: true,
					soundcloud_search_enabled: true,
					ratelimit: JukeboxRatelimit {
						ip_blocks: None,
						excluded_ips: None,
						strategy: "RotateOnBan".to_owned().into_boxed_str(),
						search_triggers_fail: true,
						retry_limit: -1,
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
