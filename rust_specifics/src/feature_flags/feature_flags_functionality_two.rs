#[cfg(not(feature = "streaming"))]
use crate::feature_flags::handler_struct::Streamer;

#[cfg(not(feature = "streaming"))]
pub struct NoopStreamer;

#[cfg(not(feature = "streaming"))]
impl Streamer for NoopStreamer {
    fn send(&self, _message: &str) {
        // Do nothing
    }
}
