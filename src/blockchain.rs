use serde::{Serialize, Deserialize};

use crate::{utils, Block, GorpCoinError, GorpCoinResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
        }
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn last_hash(&self) -> Vec<u8> {
        self.blocks
            .last()
            .map(|block| block.hash())
            .unwrap_or_else(|| vec![0])
    }

    pub fn current_difficulty(&self) -> u8 {
        difficulty_function(self.len())
    }

    pub fn add_block(&mut self, block: &Block) -> GorpCoinResult<()> {
        let hash = block.hash();
        let difficulty = self.current_difficulty();
                
        if !utils::has_valid_prefix(&hash, difficulty) {
            return Err(GorpCoinError::IncorrectDifficulty);
        }

        let expected_previous_hash = match self.blocks.last() {
            Some(previous_block) => previous_block.hash(),
            None => vec![0],
        };

        if expected_previous_hash != block.previous_hash() {
            return Err(GorpCoinError::InvalidPreviousHash);
        }

        self.blocks.push(block.clone());

        Ok(())
    }
}

fn difficulty_function(length: usize) -> u8 {
    if length == 0 {
        return 1;
    }

    let length = length as u32;
    let length = f64::from(length + 1);
    let difficulty = length.log10() + 1.0;

    difficulty as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_function() {
        assert_eq!(difficulty_function(0), 1);
        assert_eq!(difficulty_function(10), 2);
        assert_eq!(difficulty_function(50), 2);
        assert_eq!(difficulty_function(100), 3);
    }
}
