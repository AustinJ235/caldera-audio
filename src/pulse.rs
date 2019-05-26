use super::StreamID;
use super::Backend;
use std::sync::Arc;

pub struct Pulse {

}

impl Pulse {
	pub fn new() -> Self {
		Pulse {
		
		}
	}
}

impl Backend for Pulse {
	fn find_format(&self, sample_rate: Option<u32>, channels: Option<u16>) -> Result<(u32, u16), String> {
		let sample_rate = match sample_rate {
			Some(some) => if some > 192000 {
				return Err(format!("Samplerates above 192 Khz are not supported."));
			} else {
				some
			}, None => {
				unimplemented!()
			}
		};
		
		let channels = match channels {
			Some(some) => if some > 32 {
				return Err(format!("Up to 32 channels are supported."));
			} else if some == 0 {
				return Err(format!("There must be at least 1 channel."));
			} else {
				some
			}, None => {
				unimplemented!()
			}
		};
		
		Ok((sample_rate, channels))
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
