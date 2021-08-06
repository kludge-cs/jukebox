use die::die;
use jukebox::{ConfigError, Settings};

pub fn main() {
	let _settings = match Settings::new() {
		Ok(set) => set,
		Err(ConfigError::FileParse { uri, cause }) => {
			die!("Error parsing {}: {}", uri.unwrap(), cause; 115)
		}
		Err(err) => die!("Error reading config: {}", err; 115),
	};
	println!("{:?}", _settings);
}
