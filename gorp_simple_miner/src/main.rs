use std::time::Instant;
use gorp_blockchain::{Block, Blockchain};

pub fn mine_coin<T>(data: T, blockchain: &mut Blockchain<T>) 
where T: AsRef<[u8]>,
      T: Clone,
{
    let start_time = Instant::now();

    let difficulty = blockchain.current_difficulty();
    let previous_hash = blockchain.last_hash();
    let mut nonce = 0;
    
    loop {
        let block = Block::new(data.clone(), &previous_hash, nonce);
        if block.is_valid(difficulty) {
            let hash_string = block.hash_string();
            match blockchain.add_block(block) {
                Ok(_) => {
                    let duration = Instant::now().duration_since(start_time);
                    println!("Successfully added block");
                    println!("Hash: 0x{}", hash_string);
                    println!("Took: {:#?}", duration);
                    
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

pub fn main() {
    let mut blockchain = Blockchain::<String>::new();

    loop {
        let data = format!("{}", blockchain.current_difficulty());
        println!("Mining with difficulty {}", data);
        mine_coin(data, &mut blockchain);
    }
}
