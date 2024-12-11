use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> u32 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).unwrap().as_millis() as u32
}
