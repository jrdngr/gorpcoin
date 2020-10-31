use gorp_blockchain::GorpBlockchainError;

pub type GorpcoinResult<T> = Result<T, GorpcoinError>;

#[derive(Debug, Clone)]
pub enum GorpcoinError {
    InvalidTransaction,
    BlockchainError(GorpBlockchainError),
}

impl std::fmt::Display for GorpcoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GorpcoinError::InvalidTransaction => write!(f, "Invalid transaction"),
            GorpcoinError::BlockchainError(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for GorpcoinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GorpcoinError::BlockchainError(error) => Some(error),
            _ => None,
        }
    }
}

impl From<GorpBlockchainError> for GorpcoinError {
    fn from(error: GorpBlockchainError) -> Self {
        GorpcoinError::BlockchainError(error)
    }
}
