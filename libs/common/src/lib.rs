use solana_program::pubkey::Pubkey;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Invalid calculation")]
    InvalidCalculation,
    #[error("Account validation failed")]
    AccountValidationFailed,
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    #[error("Custom error: {0}")]
    Custom(String),
}

/// Common result type used across all libraries
pub type CommonResult<T> = Result<T, CommonError>;

/// Common constants used throughout the project
pub mod constants {
    pub const MAX_SEED_LENGTH: usize = 32;
    pub const DEFAULT_DECIMALS: u8 = 6;
    pub const PROGRAM_STATE_SEED: &[u8] = b"program_state";
}

/// Utility functions for working with Pubkeys
pub mod pubkey_utils {
    use super::*;

    pub fn is_valid_pubkey(pubkey: &Pubkey) -> bool {
        *pubkey != Pubkey::default()
    }

    pub fn create_program_address_safe(
        seeds: &[&[u8]], 
        program_id: &Pubkey
    ) -> CommonResult<Pubkey> {
        Pubkey::create_program_address(seeds, program_id)
            .map_err(|_| CommonError::InvalidCalculation)
    }
}

/// Common validation functions
pub mod validation {
    use super::*;

    pub fn validate_owner(account_owner: &Pubkey, expected_owner: &Pubkey) -> CommonResult<()> {
        if account_owner != expected_owner {
            return Err(CommonError::InsufficientPermissions);
        }
        Ok(())
    }

    pub fn validate_not_default(pubkey: &Pubkey) -> CommonResult<()> {
        if *pubkey == Pubkey::default() {
            return Err(CommonError::AccountValidationFailed);
        }
        Ok(())
    }
} 