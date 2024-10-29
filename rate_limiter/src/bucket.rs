use std::time::Instant;

pub struct Bucket {
    capacity: u32,
    tokens: u32,
    refill_rate: f64,
    last_refill: Instant,
}

impl Bucket {
    pub fn new() -> Self {
        Self {
            capacity: 3,
            tokens: 3,
            refill_rate: 1.0,
            last_refill: Instant::now(),
        }
    }

    pub fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let new_tokens = (elapsed * self.refill_rate) as u32;

        self.tokens = (self.tokens + new_tokens).min(self.capacity);
        self.last_refill = now;
    }

    pub fn try_acquire(&mut self) -> bool {
        self.refill();

        if self.tokens >= 1 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}
