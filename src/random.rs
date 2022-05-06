use bracket_random::prelude::RandomNumberGenerator;
use std::sync::Mutex;

/// Wraps a Random Number Generator in a mutex, so it can be safely
/// used with just Res<RNG> rather than ResMut<RNG>. This can make
/// thread-scheduling easier.
pub struct RandomNumbers {
    rng: Mutex<RandomNumberGenerator>,
}

impl RandomNumbers {
    /// Construct a new RNG
    pub fn new() -> Self {
        Self {
            rng: Mutex::new(RandomNumberGenerator::new()),
        }
    }

    // Return a number between <min> and <max>
    pub fn range(&self, min: u32, max: u32) -> u32 {
        self.rng.lock().as_mut().unwrap().range(min, max)
    }
}
