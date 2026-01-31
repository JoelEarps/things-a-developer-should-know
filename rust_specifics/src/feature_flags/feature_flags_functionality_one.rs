#[cfg(feature = "streaming")]
use crate::feature_flags::handler_struct::Streamer;

#[cfg(feature = "streaming")]
pub struct KafkaStreamer;

#[cfg(feature = "streaming")]
impl Streamer for KafkaStreamer {
    fn send(&self, message: &str) {
        println!("Sending to Kafka: {}", message);
        // actual Kafka send logic here
    }
}
