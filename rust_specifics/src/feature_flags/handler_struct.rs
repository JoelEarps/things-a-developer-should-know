pub struct AppComponentManager {
    streamer: Box<dyn Streamer + Send + Sync>,
}

impl AppComponentManager {
    pub fn new() -> Self {
        // What is Box<dyn Stream + Send + Sync> doing
        // Box - creating a item of type <dyn Stream + Send + Sync> on the heap
        // Why do we need Box ?
        // dyn Stream + Send + Sync - then item of the heap will implement Streamer, Send and Sync traits
        let streamer: Box<dyn Streamer + Send + Sync> = {
            #[cfg(feature = "streaming")]
            {
                use crate::feature_flags::feature_flags_functionality_one::KafkaStreamer;

                Box::new(KafkaStreamer)
            }

            #[cfg(not(feature = "streaming"))]
            {
                use crate::feature_flags::feature_flags_functionality_two::NoopStreamer;

                Box::new(NoopStreamer)
            }
        };

        AppComponentManager { streamer }
    }

    pub fn do_work(&self) {
        self.streamer.send("Event happened");
    }
}

pub trait Streamer {
    fn send(&self, message: &str);
}
