use std::time::{Duration, Instant};
use std::cmp::max;

#[derive(Clone)]
pub struct Frame {
    pub start_time: Instant,
    pub delta_time: f64,
}

impl Frame {
    pub fn new() -> Frame {
        let start = Instant::now();
        Frame {
            start_time: start.clone(),
            delta_time: 0.0,
        }
    }
    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }
    pub fn end(&mut self) {
        let elapsed = self.start_time.elapsed();
        self.delta_time = max(elapsed.as_millis(), 1) as f64;
    }
}