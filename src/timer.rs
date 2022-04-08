use std::time::{Duration, Instant};

pub struct Timer {
    start: Instant,
    used: Duration,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start: Instant::now(),
            used: Duration::new(0, 0),
        }
    }

    pub fn stop(&mut self) {
        self.used = self.start.elapsed();
        println!("cost: {:?}", self.used);
    }

    pub fn diff(&mut self) {
        println!("diff: {:?}", self.start.elapsed() - self.used);
        self.used = self.start.elapsed();
    }
}
