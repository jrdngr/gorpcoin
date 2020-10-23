pub type GorpCoinResult<T> = Result<T, GorpCoinError>;

#[derive(Debug, Clone)]
pub enum GorpCoinError {
    IncorrectDifficulty,
    InvalidPreviousHash,
}

impl std::fmt::Display for GorpCoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GorpCoinError::IncorrectDifficulty => write!(f, "Hash does not begin with the required number of zeros"),
            GorpCoinError::InvalidPreviousHash => write!(f, "Invalid previous hash"),
        }
    }
}

impl std::error::Error for GorpCoinError { }
