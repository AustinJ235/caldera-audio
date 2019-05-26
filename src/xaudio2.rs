use super::StreamID;
use super::Backend;
use std::sync::Arc;

pub struct XAudio2 {

}

impl XAudio2 {
	pub fn new() -> Self {
		XAudio2 {
		
		}
	}
}

impl Backend for XAudio2 {
	fn find_format(&self, sample_rate: Option<u32>, channels: Option<u16>) -> Result<(u32, u16), String> {
		unimplemented!()
	}
	
	fn open_stream(&self, sample_rate: u32, channels: u16, func: Arc<Fn(usize) -> Vec<f32> + Send + Sync>) -> Result<StreamID, String> {
		unimplemented!()
	}
	
	fn start_stream(&self, stream_id: StreamID) {
		unimplemented!()
	}
	
	fn pause_stream(&self, stream_id: StreamID) {
		unimplemented!()
	}
	
	fn close_stream(&self, stream_id: StreamID) {
		unimplemented!()
	}
	
	fn set_stream_volume(&self, stream_id: StreamID, volume: f32) {
		unimplemented!()
	}
	
	fn get_stream_volume(&self, stream_id: StreamID) -> Option<f32> {
		unimplemented!()
	}
}
