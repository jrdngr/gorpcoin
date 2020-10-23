use serde::{Serialize, Deserialize};

use crate::{Block, GorpCoinError, GorpCoinResult};

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

    pub fn current_difficulty(&self) -> u8 {
        difficulty_function(self.len())
    }

    pub fn add_block(&mut self, block: &Block) -> GorpCoinResult<()> {
        let hash = block.hash();
        let difficulty = self.current_difficulty() as usize;
        
        let valid_prefix = hash
            .iter()
            .take(difficulty)
            .all(|b| *b == 0);
        
        if !valid_prefix {
            return Err(GorpCoinError::IncorrectDifficulty);
        }

        let expected_previous_hash = match self.blocks.last() {
            Some(previous_block) => previous_block.hash(),
            None => vec![0],
        };

        if expected_previous_hash != block.previous_hash() {
            return Err(GorpCoinError::InvalidPreviousHash);
        }

        Ok(())
    }
}

fn difficulty_function(length: usize) -> u8 {
    let length = length as u32;
    let length = f64::from(length);
    let difficulty = length.log10();

    difficulty as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_function() {
        assert_eq!(difficulty_function(0), 0);
        assert_eq!(difficulty_function(10), 1);
        assert_eq!(difficulty_function(50), 1);
        assert_eq!(difficulty_function(100), 2);
    }
}
