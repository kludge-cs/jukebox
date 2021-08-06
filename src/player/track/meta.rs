pub trait Loadable {}

pub struct TrackInfo {
	title: String,
	author: String,
	length: Option<u64>,
	id: String,
	is_stream: bool,
	uri: String,
}

impl TrackInfo {
	pub fn new(
		title: String,
		author: String,
		length: Option<u64>,
		id: String,
		is_stream: bool,
		uri: String,
	) -> Self {
		TrackInfo { title, author, length, id, is_stream, uri }
	}
}

pub enum TrackState {
	Inactive,
	Loading,
	Playing,
	Seeking,
	Stopped,
	Finished,
}

pub enum TrackEndReason {
	Finished,
	LoadFailed,
	Stopped,
	Replaced,
	Cleanup,
}

impl TrackEndReason {
	pub fn can_start_next(&self) -> bool {
		match self {
			Self::Finished | Self::LoadFailed => true,
			_ => false,
		}
	}
}
