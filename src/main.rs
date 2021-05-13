mod lib;

use lib::config::Settings;

pub fn main() {
	let settings = match Settings::new() {
		Ok(set) => set,
		Err(err) => panic!("Error in config parsing: {}", err.to_string()),
	};
}
