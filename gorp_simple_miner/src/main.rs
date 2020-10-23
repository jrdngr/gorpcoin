use gorp_blockchain::{Block, Blockchain};

pub struct Miner;

impl Miner {
    pub fn mine_coin(data: &[u8], blockchain: &mut Blockchain) {
        let difficulty = blockchain.current_difficulty();
        let previous_hash = blockchain.last_hash();
        let mut nonce = 0;
        
        loop {
            let block = Block::new(data, &previous_hash, nonce);
            if block.is_valid(difficulty) {
                match blockchain.add_block(&block) {
                    Ok(_) => {
                        println!("Successfully added block with hash 0x{}", block.hash_string());
                        return;
                    },
                    Err(e) => {
                        dbg!(e);
                    }
                }
            }
            nonce += 1;
        }
    }
}

pub fn main() {
    let mut blockchain = Blockchain::new();

    let data = b"Hello!";
    Miner::mine_coin(data, &mut blockchain);

    let data = b"Hi!";
    Miner::mine_coin(data, &mut blockchain);    
}
