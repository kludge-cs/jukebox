pub mod track {
	pub mod meta;
	pub mod playback;
}

pub mod player {
	// pub mod events;
	// pub mod hook;
	mod lib;
	pub use lib::*;
}

pub mod err;

pub mod fmt {
	mod lib;
	pub use lib::*;
}
