extern crate caldera_bindings;
#[macro_use]
extern crate lazy_static;

#[cfg(target_os = "linux")]
mod pulse;
#[cfg(target_os = "windows")]
mod xaudio2;

use std::sync::Arc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

lazy_static!{
	pub static ref CAUDIO: CAudio = CAudio::new();
}

pub struct CAudio {
	backend: Box<Backend + Send + Sync>,
}

impl CAudio {
	fn new() -> Self {
		let backend = {
			#[cfg(target_os = "linux")]
			{ Box::new(pulse::Pulse::new()) as Box<Backend + Send + Sync> }
			#[cfg(target_os = "windows")]
			{ Box::new(xaudio2::XAudio2::new()) as Box<Backend + Send + Sync> }
		};
	
		CAudio {
			backend,
		}
	}
}

pub struct StreamID {

}

pub trait Backend {
	fn find_format(&self, sample_rate: Option<u32>, channels: Option<u16>) -> Result<(u32, u16), String>;
	fn open_stream(&self, sample_rate: u32, channels: u16, func: Arc<Fn(usize) -> Vec<f32> + Send + Sync>) -> Result<StreamID, String>;
	fn start_stream(&self, stream_id: StreamID);
	fn pause_stream(&self, stream_id: StreamID);
	fn close_stream(&self, stream_id: StreamID);
	fn set_stream_volume(&self, stream_id: StreamID, volume: f32);
	fn get_stream_volume(&self, stream_id: StreamID) -> Option<f32>;
}

