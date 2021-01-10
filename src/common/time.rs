use std::time::{Duration, Instant};

pub fn time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let res = f();
    (res, Instant::now() - start)
}

pub fn time_block(label: &str) -> DroppableTime {
    DroppableTime {
        label: label.to_string(),
        start: Instant::now(),
    }
}

pub struct DroppableTime {
    start: Instant,
    label: String,
}

impl Drop for DroppableTime {
    fn drop(&mut self) {
        println!("{}: {:?}", self.label, Instant::now() - self.start);
    }
}
