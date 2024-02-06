use std::time::Instant;

pub struct Performance {
    start: Instant,
}

impl Default for Performance {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl Performance {
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    pub fn elapsed(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
}
