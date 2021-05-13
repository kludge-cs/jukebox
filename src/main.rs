use jukebox::Settings;
use std::process::exit;

pub fn main() {
	let _settings = match Settings::new() {
		Ok(set) => set,
		Err(err) => {
			eprintln!("Error reading configuration: {}", err);
			exit(115);
		}
	};
}
