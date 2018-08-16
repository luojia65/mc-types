// use in daylight detector
// game time ~ inf
// world time/ 24000 a cycle
pub struct Instant {
    ticks: u64
}

pub const TICKS_PER_DAY = 24_000;

impl Instant {
    // panic if `day` overflows
    pub fn new(day: u64, time_of_day: u16) -> Instant {
        Instant {
            ticks: TICKS_PER_DAY * day + time_of_day
        }
    }
    // use in daylight detector
    pub fn time_of_day(&self) -> u16 {
        self.ticks % TICKS_PER_DAY;
    }
}

// moon phases

// to realtime clock?
