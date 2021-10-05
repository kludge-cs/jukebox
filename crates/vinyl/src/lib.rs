#![deny(missing_docs)]

pub mod track {
	mod lib;
	pub mod meta;
	pub mod playback;
	pub use lib::*;
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

pub mod filter {
	mod lib;
	pub use lib::*;
}
