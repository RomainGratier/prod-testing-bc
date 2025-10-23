use thiserror::Error;

#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Block validation failed: {0}")]
    BlockValidationFailed(String),
    
    #[error("Insufficient balance for transaction")]
    InsufficientBalance,
    
    #[error("Transaction already exists")]
    DuplicateTransaction,
    
    #[error("Block already exists")]
    DuplicateBlock,
    
    #[error("Performance limit exceeded: {0}")]
    PerformanceLimitExceeded(String),
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, LedgerError>;