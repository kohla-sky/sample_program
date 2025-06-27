use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

// Import our path dependencies
use account_utils::{account_creation, account_validation, account_data};
use math_utils::{token_math, percentage, safe_math};

// This also brings in common transitively through our dependencies
use common::{CommonResult, CommonError};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        ProgramInstruction::Initialize { initial_amount } => {
            msg!("Instruction: Initialize");
            process_initialize(program_id, accounts, initial_amount)
        }
        ProgramInstruction::CreateUserAccount { initial_balance } => {
            msg!("Instruction: CreateUserAccount");
            process_create_user_account(program_id, accounts, initial_balance)
        }
        ProgramInstruction::TransferWithFee { amount, fee_basis_points } => {
            msg!("Instruction: TransferWithFee");
            process_transfer_with_fee(program_id, accounts, amount, fee_basis_points)
        }
    }
}

fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let program_state_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    
    // Use math-utils to calculate token amount with default decimals
    let token_amount = token_math::calculate_default_token_amount(initial_amount)
        .map_err(|_| ProgramError::InvalidArgument)?;
    
    // Create program state using account-utils
    let (expected_pda, _bump) = account_creation::create_program_state_pda(program_id)
        .map_err(|_| ProgramError::InvalidSeeds)?;
    
    if program_state_info.key != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }
    
    // Validate accounts using account-utils
    account_validation::validate_signer(payer_info)
        .map_err(|_| ProgramError::MissingRequiredSignature)?;
    
    let program_state = ProgramState {
        authority: *payer_info.key,
        total_supply: token_amount,
        is_initialized: true,
    };
    
    // Serialize and save the program state
    let data = program_state.try_to_vec()
        .map_err(|_| ProgramError::BorshIoError("Failed to serialize program state".to_string()))?;
    
    program_state_info.data.borrow_mut()[..data.len()].copy_from_slice(&data);
    
    msg!("Program initialized with total supply: {}", token_amount);
    Ok(())
}

fn process_create_user_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_balance: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user_account_info = next_account_info(account_info_iter)?;
    let user_info = next_account_info(account_info_iter)?;
    let program_state_info = next_account_info(account_info_iter)?;
    
    // Use math-utils for safe arithmetic
    let balance = safe_math::safe_mul(initial_balance, 1000)
        .map_err(|_| ProgramError::InvalidArgument)?;
    
    // Validate using account-utils
    account_validation::validate_signer(user_info)
        .map_err(|_| ProgramError::MissingRequiredSignature)?;
    
    // Create user PDA using account-utils
    let (expected_pda, _bump) = account_creation::create_user_pda(user_info.key, program_id)
        .map_err(|_| ProgramError::InvalidSeeds)?;
    
    if user_account_info.key != &expected_pda {
        return Err(ProgramError::InvalidSeeds);
    }
    
    let user_account = UserAccount {
        owner: *user_info.key,
        balance,
        program_state: *program_state_info.key,
    };
    
    // Serialize and save the user account
    let data = user_account.try_to_vec()
        .map_err(|_| ProgramError::BorshIoError("Failed to serialize user account".to_string()))?;
    
    user_account_info.data.borrow_mut()[..data.len()].copy_from_slice(&data);
    
    msg!("User account created with balance: {}", balance);
    Ok(())
}

fn process_transfer_with_fee(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    fee_basis_points: u16,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from_account_info = next_account_info(account_info_iter)?;
    let to_account_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    
    // Validate signer
    account_validation::validate_signer(owner_info)
        .map_err(|_| ProgramError::MissingRequiredSignature)?;
    
    // Deserialize accounts using account-utils
    let mut from_account = account_data::deserialize_account_data::<UserAccount>(from_account_info)
        .map_err(|_| ProgramError::InvalidAccountData)?;
    
    let mut to_account = account_data::deserialize_account_data::<UserAccount>(to_account_info)
        .map_err(|_| ProgramError::InvalidAccountData)?;
    
    // Validate ownership
    if from_account.owner != *owner_info.key {
        return Err(ProgramError::InvalidArgument);
    }
    
    // Calculate fee using math-utils percentage module
    let fee = percentage::calculate_percentage(amount, fee_basis_points)
        .map_err(|_| ProgramError::InvalidArgument)?;
    
    let total_amount = safe_math::safe_add(amount, fee)
        .map_err(|_| ProgramError::InvalidArgument)?;
    
    // Validate sufficient balance
    if from_account.balance < total_amount {
        return Err(ProgramError::InsufficientFunds);
    }
    
    // Perform transfer using safe math
    from_account.balance = safe_math::safe_sub(from_account.balance, total_amount)
        .map_err(|_| ProgramError::InvalidArgument)?;
    
    to_account.balance = safe_math::safe_add(to_account.balance, amount)
        .map_err(|_| ProgramError::InvalidArgument)?;
    
    // Serialize and save the updated accounts
    let from_data = from_account.try_to_vec()
        .map_err(|_| ProgramError::BorshIoError("Failed to serialize from account".to_string()))?;
    
    let to_data = to_account.try_to_vec()
        .map_err(|_| ProgramError::BorshIoError("Failed to serialize to account".to_string()))?;
    
    from_account_info.data.borrow_mut()[..from_data.len()].copy_from_slice(&from_data);
    to_account_info.data.borrow_mut()[..to_data.len()].copy_from_slice(&to_data);
    
    msg!("Transferred {} tokens with fee: {}", amount, fee);
    Ok(())
}

/// Program instruction enum
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ProgramInstruction {
    /// Initialize the program state
    Initialize {
        initial_amount: u64,
    },
    /// Create a user account
    CreateUserAccount {
        initial_balance: u64,
    },
    /// Transfer tokens between users with fee calculation
    TransferWithFee {
        amount: u64,
        fee_basis_points: u16,
    },
}

/// Program state account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ProgramState {
    pub authority: Pubkey,
    pub total_supply: u64,
    pub is_initialized: bool,
}

/// User account structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserAccount {
    pub owner: Pubkey,
    pub balance: u64,
    pub program_state: Pubkey,
} 