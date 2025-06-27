use solana_program::{
    pubkey::Pubkey,
    account_info::AccountInfo,
};
use borsh::{BorshSerialize, BorshDeserialize};
use common::{
    CommonError, CommonResult, 
    validation,
    constants::{MAX_SEED_LENGTH, PROGRAM_STATE_SEED}
};

/// Account creation and validation utilities
pub mod account_creation {
    use super::*;

    /// Create a PDA (Program Derived Address) with validation
    pub fn create_pda_with_validation(
        seeds: &[&[u8]], 
        program_id: &Pubkey
    ) -> CommonResult<(Pubkey, u8)> {
        // Validate seed lengths
        for seed in seeds {
            if seed.len() > MAX_SEED_LENGTH {
                return Err(CommonError::Custom("Seed too long".to_string()));
            }
        }

        let (pubkey, bump) = Pubkey::find_program_address(seeds, program_id);
        validation::validate_not_default(&pubkey)?;
        Ok((pubkey, bump))
    }

    /// Create program state PDA
    pub fn create_program_state_pda(program_id: &Pubkey) -> CommonResult<(Pubkey, u8)> {
        create_pda_with_validation(&[PROGRAM_STATE_SEED], program_id)
    }

    /// Create user-specific PDA
    pub fn create_user_pda(
        user_pubkey: &Pubkey, 
        program_id: &Pubkey
    ) -> CommonResult<(Pubkey, u8)> {
        validation::validate_not_default(user_pubkey)?;
        create_pda_with_validation(&[b"user", user_pubkey.as_ref()], program_id)
    }
}

/// Account validation utilities
pub mod account_validation {
    use super::*;

    /// Validate account info structure
    pub fn validate_account_info(account_info: &AccountInfo) -> CommonResult<()> {
        validation::validate_not_default(account_info.key)?;
        
        if account_info.data_is_empty() {
            return Err(CommonError::AccountValidationFailed);
        }
        
        Ok(())
    }

    /// Validate signer account
    pub fn validate_signer(account_info: &AccountInfo) -> CommonResult<()> {
        if !account_info.is_signer {
            return Err(CommonError::InsufficientPermissions);
        }
        validate_account_info(account_info)
    }

    /// Validate writable account
    pub fn validate_writable(account_info: &AccountInfo) -> CommonResult<()> {
        if !account_info.is_writable {
            return Err(CommonError::InsufficientPermissions);
        }
        validate_account_info(account_info)
    }

    /// Validate account owner
    pub fn validate_account_owner(
        account_info: &AccountInfo, 
        expected_owner: &Pubkey
    ) -> CommonResult<()> {
        validation::validate_owner(account_info.owner, expected_owner)?;
        validate_account_info(account_info)
    }
}

/// Account data management
pub mod account_data {
    use super::*;

    /// Safely deserialize account data
    pub fn deserialize_account_data<T: BorshDeserialize>(
        account_info: &AccountInfo
    ) -> CommonResult<T> {
        account_validation::validate_account_info(account_info)?;
        
        T::try_from_slice(&account_info.data.borrow())
            .map_err(|_| CommonError::AccountValidationFailed)
    }

    /// Calculate required account size
    pub fn calculate_account_size<T: BorshSerialize>(
        data: &T
    ) -> CommonResult<usize> {
        data.try_to_vec()
            .map(|vec| vec.len())
            .map_err(|_| CommonError::InvalidCalculation)
    }

    /// Validate account has sufficient space
    pub fn validate_account_space(
        account_info: &AccountInfo, 
        required_size: usize
    ) -> CommonResult<()> {
        if account_info.data_len() < required_size {
            return Err(CommonError::AccountValidationFailed);
        }
        Ok(())
    }
} 