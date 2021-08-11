use crate::err::PlayerFrameError;

// TODO: implement audio formats
pub type AudioFormat = Box<str>;

pub struct Frame {
	pub timecode: u64, // time in ms
	pub volume: u8,
	pub data: Vec<u8>, // byte array of frame data
	pub format: AudioFormat,
	pub terminator: bool,
}

pub trait FrameProvider {
	fn provide(self) -> Option<Frame>;
	fn provide_with(
		self,
		timeout: Option<u64>,
		unit: Option<&str>,
	) -> Result<Frame, PlayerFrameError>;
	fn provide_mut(self, frame: &mut Frame) -> bool;
	fn provide_mut_with(
		self,
		frame: &mut Frame,
		timeout: Option<u64>,
		unit: Option<&str>,
	) -> Result<bool, PlayerFrameError>;
}

pub trait FrameRebuilder {
	fn rebuild(frame: Frame) -> Frame;
}

pub trait FrameConsumer {
	fn consume(self, frame: Frame) -> Result<(), PlayerFrameError>;
	fn rebuild<T: FrameRebuilder>(self, rebuilder: T);
}

pub trait FrameBuffer: FrameProvider + FrameConsumer {
	fn new(duration: u8, format: AudioFormat, stopping: bool) -> Self;
	fn remaining_capacity(&self) -> u8;
	fn full_capacity(&self) -> u8;
	fn wait_for_termination(&self) -> Result<(), PlayerFrameError>;
	fn terminate_on_empty(&self);
	fn clear_on_insert(self);
	fn will_clear_on_insert(self) -> bool;
	fn clear(self);
	fn has_received(&self) -> bool;
	fn last_input_timecode(&self) -> Option<u64>;
}
