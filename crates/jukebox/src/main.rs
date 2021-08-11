use fail::fail;
use jukebox::{ConfigError, Settings};

pub fn main() {
	let _settings = match Settings::new() {
		Ok(set) => set,
		Err(ConfigError::FileParse { uri, cause }) => {
			fail!("Error parsing {}: {}", uri.unwrap(), cause; 115)
		}
		Err(err) => fail!("Error reading config: {}", err; 115),
	};
	println!("{:?}", _settings);
}
