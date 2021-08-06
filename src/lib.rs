mod util {
	pub mod config;
}
pub use util::config::{ConfigError, Settings};

pub mod player {
	pub mod track {
		pub mod meta;
		pub mod playback;
	}
}

mod server {}

pub mod err;
