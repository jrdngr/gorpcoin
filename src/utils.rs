use std::time::{SystemTime, UNIX_EPOCH};

pub fn unix_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    }
}

pub fn has_valid_prefix(hash: &[u8], difficulty: u8) -> bool {
    let difficulty = difficulty as usize;

    hash
        .iter()
        .take(difficulty)
        .all(|b| *b == 0)
}
