use die::die;
use jukebox::Settings;

pub fn main() {
	let _settings = match Settings::new() {
		Ok(set) => set,
		Err(err) => die!("Error reading configuration: {}", err; 115),
	};
}
