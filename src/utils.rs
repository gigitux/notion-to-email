use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_random_index(max: usize) -> usize {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let nanos = since_the_epoch.as_nanos();
    (nanos % max as u128) as usize
}
