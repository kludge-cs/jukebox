use crate::{
	err::PlayerFrameError, fmt::AudioDataFmt,
	track::playback::ProcessingContext,
};
use bytes::{Buf, BufMut, BytesMut};

/// Converts an `f32` audio sample to an `i16` for 16-bit LPCM.
pub fn decode_sample(sample: f32) -> i16 {
	i16::min(i32::max((sample * 32768f32) as i32, -32768) as i16, 32767)
}

pub trait Filter {
	/// Indicates that the next samples are not continuous from the prior
	/// samples.
	///
	/// # Arguments
	/// * `requested_time` - Timecode in ms the seek was requested at
	/// * `provided_time` - Timecode in ms the seek was actually performed at
	fn seek_performed(&mut self, requested_time: u64, provided_time: u64);
	/// Flushes everything to output.
	///
	///
	fn flush(&self) -> Result<(), PlayerFrameError>;
	fn close(&self);
}

pub trait PCMFilter: Filter {
	fn process_at(
		&self,
		input: &[Box<dyn PCMData>],
		offset: u32,
		length: u32,
	) -> Result<(), PlayerFrameError>;
	fn process_all(
		&self,
		buffer: &[Box<dyn PCMData>],
	) -> Result<(), PlayerFrameError>;
}

pub trait SplitPCMFilter: Filter {
	fn process_channels(
		&self,
		input: &[&[Box<dyn PCMData>]],
		offset: u32,
		length: u32,
	) -> Result<(), PlayerFrameError>;
}

pub trait PCMFilterFactory {
	/// Builds a filter chain for processing a track. This may be called several
	/// times during playback. All filters should send the output to the next
	/// filter in the vector, or to the output filter if there is no next filter.
	///
	/// # Arguments
	/// * `track` - The track this chain was built for
	/// * `format` - The output format of the track.
	/// * `output` - The aforementioned output filter.
	fn build_chain(
		&self,
		track: Option<()>,
		fmt: AudioDataFmt,
		output: Box<dyn UniversalPCMFilter>,
	) -> Vec<Box<dyn Filter>>;
}

pub trait PCMData {}
impl PCMData for i16 {}
impl PCMData for f32 {}

pub trait UniversalPCMFilter: PCMFilter + SplitPCMFilter {}

pub struct FinalPCMFilter {
	fmt: AudioDataFmt,
	buf: BytesMut,
	post_processors: Vec<()>,
	ignored_frames: u64,
	timecode_base: u64,
	timecode_offset: u64,
}

impl FinalPCMFilter {
	pub fn new(context: ProcessingContext, post_processors: Vec<()>) -> Self {
		let fmt = context.output_fmt;
		FinalPCMFilter {
			fmt,
			buf: BytesMut::with_capacity(fmt.frame_sample_count() as usize * 2),
			post_processors,
			ignored_frames: 0,
			timecode_base: 0,
			timecode_offset: 0,
		}
	}

	fn fill_frame_buf(&self) {
		self.buf.put_bytes(0, self.buf.remaining_mut());
	}

	fn dispatch(&self) -> Result<(), PlayerFrameError> {
		if !self.buf.has_remaining() {
			let timecode = self.timecode_base
				+ self.timecode_offset * 1000 / self.fmt.sample_rate;
			self.buf.clear();

			for post_processor in self.post_processors {
				todo!("implement postprocessing");
				// post_processor.process(timecode, self.buf);
			}

			self.buf.clear();
			self.timecode_offset += self.fmt.chunk_sample_count;
		}
		Ok(())
	}
}

impl Filter for FinalPCMFilter {
	fn seek_performed(&mut self, requested_time: u64, provided_time: u64) {
		self.buf.clear();
		self.ignored_frames = if requested_time > provided_time {
			(requested_time - provided_time)
				* self.fmt.channel_count
				* self.fmt.sample_rate
				/ 1000
		} else {
			0
		};
		self.timecode_base = u64::max(requested_time, provided_time);
		self.timecode_offset = 0;

		if self.ignored_frames > 0 {
			todo!("implement logging");
		}
	}

	fn flush(&self) -> Result<(), PlayerFrameError> {
		if !self.buf.is_empty() {
			self.fill_frame_buf();
			self.dispatch()?;
		}
	}

	fn close(&self) {
		for post_processor in self.post_processors {
			todo!("implement postprocessors");
			// post_processor.close();
		}
	}
}

impl PCMFilter for FinalPCMFilter {}

impl SplitPCMFilter for FinalPCMFilter {}

impl UniversalPCMFilter for FinalPCMFilter {}
