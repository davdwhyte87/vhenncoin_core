use std::error::Error;
use thiserror::Error;
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error")]
    ValidationError,
    
    // database 
    #[error("Error connecting to database {0}")]
    CreateDatabaseError(String),
    #[error("Error inserting data {0}")]
    DatabaseInsertError(String),
    #[error("Database error {0}")]
    OpenTableError(String),
    #[error("Error saving data .. {0}")]
    ErrorInsertingData(String),
    
    #[error("Hex decode error {0}")]
    HexDecodeError(String),
    #[error("Verify key error {0}")]
    VerifyKeyError(String),
    #[error("error with signature {0}")]
    SignatureError(String),
    #[error("tx Id mismatch ")]
    IDMismatch,
    
    // account errors
    #[error("Account not found -- {0}")]
    AccountNotFound(String),
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Transaction exists")]
    TransactionAlreadyExists,
    
    // serialization errors
    #[error("Serialization Error {0}")]
    SerializationError(String),
    #[error("Error with big decimal conversion {0}")]
    BigDecimalConversionError(String),
    
    #[error("Error occurred {0}")]
    UnexpectedError(String )
}

