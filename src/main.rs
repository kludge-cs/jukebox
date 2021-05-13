use jukebox::Settings;

pub fn main() {
	let _settings = match Settings::new() {
		Ok(set) => set,
		Err(err) => panic!("Error in config parsing: {}", err.to_string()),
	};
}
