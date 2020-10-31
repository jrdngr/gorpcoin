pub mod block_data;
pub mod error;
pub mod transaction;

use gorp_blockchain::Blockchain;

pub use block_data::GorpcoinBlockData;
pub use transaction::{Transaction, TransactionData};

pub type Hash = String;
pub type Address = String;

pub type GorpcoinBlockchain = Blockchain<GorpcoinBlockData>;

pub fn is_transaction_valid(_blockchain: GorpcoinBlockchain, _transaction: &Transaction) -> bool {
    false
}
