use std::error::Error;
type Track = Option<()>;

pub trait StateListener {
	fn on_track_exception(&self, track: Track, exception: dyn Error);
	fn on_track_stuck(&self, track: Track, threshold_ms: u64);
}

pub struct Marker {
	pub timecode: u64,
	pub handler: Box<dyn MarkerHandler>,
}

impl Marker {
	pub fn new(timecode: u64, handler: impl MarkerHandler) -> Self {
		Marker { timecode, handler }
	}
}

pub trait MarkerHandler {
	fn handle(&self, state: MarkerState);
}

pub enum MarkerState {
	REACHED,
	REMOVED,
	OVERWRITTEN,
	BYPASSED,
	STOPPED,
	LATE,
	ENDED,
}
