pub type GorpcoinResult<T> = Result<T, GorpcoinError>;

#[derive(Debug, Clone)]
pub enum GorpcoinError {
    IncorrectDifficulty,
    InvalidPreviousHash,
}

impl std::fmt::Display for GorpcoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GorpcoinError::IncorrectDifficulty => {
                write!(f, "Hash does not begin with the required number of zeros")
            }
            GorpcoinError::InvalidPreviousHash => write!(f, "Invalid previous hash"),
        }
    }
}

impl std::error::Error for GorpcoinError {}
