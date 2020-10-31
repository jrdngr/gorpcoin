use gorpcoin::{GorpcoinBlock, GorpcoinBlockData, GorpcoinBlockchain};
use std::time::Instant;

pub fn mine_coin(data: GorpcoinBlockData, blockchain: &mut GorpcoinBlockchain) {
    let start_time = Instant::now();

    let difficulty = blockchain.current_difficulty();
    let previous_hash = blockchain.last_hash();
    let mut nonce = 0;

    loop {
        let block = GorpcoinBlock::new(data.clone(), &previous_hash, nonce);
        if block.is_valid(difficulty) {
            let hash_string = block.hash_string();
            match blockchain.add_block(block) {
                Ok(_) => {
                    let duration = Instant::now().duration_since(start_time);
                    println!("Successfully added block");
                    println!("Hash: 0x{}", hash_string);
                    println!("Took: {:#?}", duration);

                    return;
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
        nonce += 1;
    }
}

pub fn main() {
    let mut blockchain = GorpcoinBlockchain::new();

    loop {
        let data = format!("{}", blockchain.current_difficulty());
        println!("Mining with difficulty {}", data);
        mine_coin(data, &mut blockchain);
    }
}
