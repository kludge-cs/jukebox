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
	INACTIVE,
	LOADING,
	PLAYING,
	SEEKING,
	STOPPING,
	FINISHED,
}

pub enum TrackEndReason {
	FINISHED,
	LOAD_FAILED,
	STOPPED,
	REPLACED,
	CLEANUP,
}

impl TrackEndReason {
	pub fn start_next(&self) -> bool {
		match self {
			Self::FINISHED | Self::LOAD_FAILED => true,
			_ => false,
		}
	}
}
