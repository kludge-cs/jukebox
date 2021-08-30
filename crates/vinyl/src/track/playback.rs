use std::sync::atomic::AtomicBool;

use crate::{
	err::PlayerFrameError,
	fmt::AudioFmt,
	player::{AudioConfig, AudioPlayerOpts},
	track::meta::TrackState,
};

// TODO: implement these (part of main track, not playback)
pub type StateListener = Option<()>;
pub type TrackMarker = Option<()>;

pub struct Frame {
	pub timecode: u64, // time in ms
	pub volume: u8,
	pub data: Vec<u8>, // byte array of frame data
	pub format: Box<dyn AudioFmt>,
	pub terminator: bool,
}

pub struct ProcessingContext {
	pub config: AudioConfig,
	pub buf: Box<dyn FrameBuf>,
	pub opts: AudioPlayerOpts,
	pub hotswap: bool,
	pub output_fmt: Box<dyn AudioFmt>,
}

impl ProcessingContext {
	pub fn new(
		config: AudioConfig,
		buf: Box<dyn FrameBuf>,
		opts: AudioPlayerOpts,
		output_fmt: Box<dyn AudioFmt>,
	) -> Self {
		let hotswap = config.hotswap.to_owned();
		ProcessingContext { config, buf, opts, hotswap, output_fmt }
	}
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
	fn rebuild(self, frame: Frame) -> Frame;
}

pub trait FrameConsumer {
	fn consume(self, frame: Frame) -> Result<(), PlayerFrameError>;
	fn rebuild(self, rebuilder: dyn FrameRebuilder) -> Frame;
}

pub trait FrameBuf: FrameProvider + FrameConsumer {
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

pub trait FrameBufFactory {
	fn create(
		&self,
		buf_duration: u32,
		fmt: Box<dyn AudioFmt>,
		stopping: AtomicBool,
	) -> dyn FrameBuf;
}

pub trait TrackExecutor: FrameProvider {
	fn audio_buffer(&self) -> dyn FrameBuf;
	fn execute(&self, listener: StateListener);
	fn stop(&self);
	fn curr_pos(&self) -> u64;
	fn seek(&self, timecode: u64) -> u64;
	fn state(&self) -> TrackState;
	fn mark(&self, marker: TrackMarker);
	fn failed_before_load(&self) -> bool;
}
