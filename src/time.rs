use std::rc::Rc;
use std::time::{Duration, Instant};

use logger::Logger;

pub struct Time {
    logger: Rc<Logger>,
    init_time: Instant,
    time_of_last_update: Instant,
    last_delta_time: f64,   // Time in secondss
}

impl Time {
    pub fn new(logger: Rc<Logger>) -> Self {
        let init_time = Instant::now();
        let time_of_last_update = Instant::now();
        let last_delta_time = 0f64;

        Self {
            logger,
            init_time,
            time_of_last_update,
            last_delta_time,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        let duration_elapsed = self.time_of_last_update.elapsed();
        self.last_delta_time = duration_elapsed.as_secs() as f64 
                             + duration_elapsed.subsec_nanos() as f64 
                             / 1e9;

        info!(self.logger, "FPS: {}, Frame time: {}s", 1f64/self.last_delta_time, self.last_delta_time);

        self.time_of_last_update = now;
    }

    pub fn delta_time(&self) -> f64 {
        self.last_delta_time
    }
}