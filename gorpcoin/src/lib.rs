pub mod blockchain;
pub mod error;
pub mod transaction;
pub mod utils;

pub use blockchain::GorpcoinBlockchain;
pub use transaction::{Transaction, TransactionData};

pub type Hash = String;
pub type Address = String;

pub fn is_transaction_valid(_blockchain: &GorpcoinBlockchain, _transaction: &Transaction) -> bool {
    false
}
