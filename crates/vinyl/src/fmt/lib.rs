use std::hash::{Hash, Hasher};

use crate::player::AudioConfig;

pub trait AudioFmt {
	fn frame_sample_count(&self) -> u32;
	fn frame_duration(&self) -> u64;
	fn codec_name(&self) -> String;
	fn silence_bytes(&self) -> &[u8];
	fn expected_chunk_size(&self) -> u32;
	fn maximum_chunk_size(&self) -> u32;
	fn create_decoder(&self) -> Box<str>;
	fn create_encoder(&self, config: AudioConfig) -> Box<str>;
}

#[derive(Eq)]
pub struct AudioDataFmt {
	pub channel_count: u32,
	pub sample_rate: u32,
	pub chunk_sample_count: u32,
}

impl AudioDataFmt {
	pub fn new(
		channel_count: u32,
		sample_rate: u32,
		chunk_sample_count: u32,
	) -> AudioDataFmt {
		AudioDataFmt { channel_count, sample_rate, chunk_sample_count }
	}
}

impl AudioFmt for AudioDataFmt {
	fn frame_sample_count(&self) -> u32 {
		self.chunk_sample_count * self.channel_count
	}
	fn frame_duration(&self) -> u64 {
		(self.chunk_sample_count * 1000 / self.sample_rate) as u64
	}
	fn codec_name(&self) -> String {
		unimplemented!();
	}
	fn silence_bytes(&self) -> &[u8] {
		unimplemented!()
	}
	fn expected_chunk_size(&self) -> u32 {
		unimplemented!()
	}
	fn maximum_chunk_size(&self) -> u32 {
		unimplemented!()
	}
	fn create_decoder(&self) -> Box<str> {
		unimplemented!()
	}
	fn create_encoder(&self, _config: AudioConfig) -> Box<str> {
		unimplemented!()
	}
}

impl PartialEq for AudioDataFmt {
	fn eq(&self, other: &Self) -> bool {
		self.channel_count == other.channel_count
			&& self.sample_rate == other.sample_rate
			&& self.chunk_sample_count == other.chunk_sample_count
			&& self.codec_name() == other.codec_name()
	}
}

impl Hash for AudioDataFmt {
	fn hash<H: Hasher>(&self, state: &mut H) {
		state.write_u32(self.channel_count);
		state.write_u32(self.sample_rate);
		state.write_u32(self.chunk_sample_count);
		state.write(self.codec_name().as_bytes());
	}
}
