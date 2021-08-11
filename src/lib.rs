pub mod util {
	pub mod config;
}
pub use util::config::{ConfigError, Settings};

pub use vinyl as player;

pub mod server {}

pub mod err;
