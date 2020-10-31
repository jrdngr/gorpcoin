pub type GorpBlockchainResult<T> = Result<T, GorpBlockchainError>;

#[derive(Debug, Clone)]
pub enum GorpBlockchainError {
    IncorrectDifficulty,
    InvalidPreviousHash,
}

impl std::fmt::Display for GorpBlockchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GorpBlockchainError::IncorrectDifficulty => {
                write!(f, "Hash does not begin with the required number of zeros")
            }
            GorpBlockchainError::InvalidPreviousHash => write!(f, "Invalid previous hash"),
        }
    }
}

impl std::error::Error for GorpBlockchainError {}
