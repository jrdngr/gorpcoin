use std::time::{SystemTime, UNIX_EPOCH};

pub fn unix_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    }
}

pub fn has_valid_prefix(hash: &[u8], difficulty: u8) -> bool {
    if difficulty == 0 {
        return true;
    }

    let difficulty = difficulty as usize;

    hash.iter().take(difficulty).all(|b| *b == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_prefix() {
        let hash = vec![1, 2, 3];
        assert!(!has_valid_prefix(&hash, 1));

        let hash = vec![0, 1, 2];
        assert!(has_valid_prefix(&hash, 1));

        let hash = vec![0, 0, 2];
        assert!(has_valid_prefix(&hash, 1));

        let hash = vec![0, 0, 2];
        assert!(has_valid_prefix(&hash, 2));
    }
}
