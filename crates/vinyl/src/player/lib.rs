use std::sync::{atomic::AtomicU16, Arc};

use crate::{
	filter::PCMFilterFactory,
	fmt::{AudioDataFmt, AudioFmt},
	track::playback::FrameBufFactory,
};

pub const OPUS_QUALITY_MAX: u8 = 10;

pub enum ResamplingQuality {
	HIGH,
	MEDIUM,
	LOW,
}

pub struct AudioConfig {
	pub resample_quality: ResamplingQuality,
	pub opus_quality: u8,
	pub output_fmt: Box<dyn AudioFmt>,
	pub hotswap: bool,
	pub buf_factory: Option<Box<dyn FrameBufFactory>>,
}

impl Default for AudioConfig {
	fn default() -> Self {
		AudioConfig {
			resample_quality: ResamplingQuality::LOW,
			opus_quality: OPUS_QUALITY_MAX,
			output_fmt: Box::new(AudioDataFmt::new(0, 0, 0)),
			hotswap: false,
			buf_factory: None,
		}
	}
}

pub struct AudioPlayerOpts {
	pub volume: AtomicU16,
	pub filter_factory: Arc<Option<Box<dyn PCMFilterFactory>>>,
	pub buf_duration: Arc<Option<u32>>,
}

impl AudioPlayerOpts {
	pub fn new() -> Self {
		AudioPlayerOpts {
			volume: AtomicU16::new(100u16),
			filter_factory: Arc::new(None),
			buf_duration: Arc::new(None),
		}
	}
}
