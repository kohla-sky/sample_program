use solana_program::{
    pubkey::Pubkey,
    keccak,
};
use common::{CommonError, CommonResult, constants::MAX_SEED_LENGTH};

/// Cryptographic hashing utilities for account operations
pub mod hashing {
    use super::*;

    /// Generate a deterministic hash from account data
    pub fn hash_account_data(data: &[u8]) -> [u8; 32] {
        keccak::hash(data).to_bytes()
    }

    /// Create a hash-based identifier for account validation
    pub fn create_account_identifier(owner: &Pubkey, seed: &[u8]) -> [u8; 32] {
        let mut combined = Vec::new();
        combined.extend_from_slice(owner.as_ref());
        combined.extend_from_slice(seed);
        hash_account_data(&combined)
    }

    /// Verify account data integrity using hash comparison
    pub fn verify_account_integrity(data: &[u8], expected_hash: &[u8; 32]) -> CommonResult<()> {
        let computed_hash = hash_account_data(data);
        if computed_hash != *expected_hash {
            return Err(CommonError::AccountValidationFailed);
        }
        Ok(())
    }

    /// Generate a unique salt for account operations
    pub fn generate_account_salt(base_pubkey: &Pubkey, nonce: u64) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(base_pubkey.as_ref());
        data.extend_from_slice(&nonce.to_le_bytes());
        hash_account_data(&data)
    }
}

/// Seed generation utilities for PDA creation
pub mod seed_generation {
    use super::*;
    use common::constants::MAX_SEED_LENGTH;

    /// Generate deterministic seeds for PDA creation
    pub fn generate_deterministic_seed(
        base: &Pubkey,
        identifier: &str,
        nonce: u32
    ) -> CommonResult<Vec<u8>> {
        let mut seed = Vec::new();
        
        // Add truncated pubkey (first 16 bytes to save space)
        seed.extend_from_slice(&base.as_ref()[..16]);
        
        // Add identifier bytes (truncated if too long)
        let id_bytes = identifier.as_bytes();
        let id_len = std::cmp::min(id_bytes.len(), 8);
        seed.extend_from_slice(&id_bytes[..id_len]);
        
        // Add nonce
        seed.extend_from_slice(&nonce.to_le_bytes());
        
        if seed.len() > MAX_SEED_LENGTH {
            return Err(CommonError::Custom("Generated seed too long".to_string()));
        }
        
        Ok(seed)
    }

    /// Create a hierarchical seed structure for nested accounts
    pub fn create_hierarchical_seed(
        parent: &Pubkey,
        child_type: u8,
        child_index: u16
    ) -> CommonResult<Vec<u8>> {
        let mut seed = Vec::new();
        
        // Use last 20 bytes of parent for uniqueness
        seed.extend_from_slice(&parent.as_ref()[12..]);
        seed.push(child_type);
        seed.extend_from_slice(&child_index.to_le_bytes());
        
        if seed.len() > MAX_SEED_LENGTH {
            return Err(CommonError::Custom("Hierarchical seed too long".to_string()));
        }
        
        Ok(seed)
    }

    /// Generate a time-based seed for temporary accounts
    pub fn generate_temporal_seed(base: &Pubkey, timestamp: i64) -> Vec<u8> {
        let mut seed = Vec::new();
        
        // Use middle portion of pubkey for temporal accounts
        seed.extend_from_slice(&base.as_ref()[8..24]);
        
        // Add timestamp for uniqueness
        seed.extend_from_slice(&timestamp.to_le_bytes());
        
        seed
    }
}

/// Address derivation utilities specific to account management
pub mod address_derivation {
    use super::*;

    /// Derive a secondary address from a primary account
    pub fn derive_secondary_address(
        primary: &Pubkey,
        derivation_path: &[u8],
        program_id: &Pubkey
    ) -> CommonResult<(Pubkey, u8)> {
        if derivation_path.len() > MAX_SEED_LENGTH {
            return Err(CommonError::Custom("Derivation path too long".to_string()));
        }
        
        let mut seeds = Vec::new();
        seeds.push(b"secondary".as_ref());
        seeds.push(primary.as_ref());
        seeds.push(derivation_path);
        
        let seed_refs: Vec<&[u8]> = seeds.iter().map(|s| s.as_ref()).collect();
        
        Ok(Pubkey::find_program_address(&seed_refs, program_id))
    }

    /// Create a unique address for account metadata storage
    pub fn derive_metadata_address(
        account: &Pubkey,
        metadata_type: &str,
        program_id: &Pubkey
    ) -> CommonResult<(Pubkey, u8)> {
        let type_hash = hashing::hash_account_data(metadata_type.as_bytes());
        
        let seeds = [
            b"metadata".as_ref(),
            account.as_ref(),
            &type_hash[..8], // Use first 8 bytes of hash
        ];
        
        Ok(Pubkey::find_program_address(&seeds, program_id))
    }

    /// Derive a vault address for secure storage
    pub fn derive_vault_address(
        owner: &Pubkey,
        vault_id: u64,
        program_id: &Pubkey
    ) -> CommonResult<(Pubkey, u8)> {
        let seeds = [
            b"vault".as_ref(),
            owner.as_ref(),
            &vault_id.to_le_bytes(),
        ];
        
        Ok(Pubkey::find_program_address(&seeds, program_id))
    }
}

/// Account validation using cryptographic proofs
pub mod validation {
    use super::*;

    /// Validate account ownership using cryptographic proof
    pub fn validate_account_ownership_proof(
        account: &Pubkey,
        owner: &Pubkey,
        proof: &[u8; 32]
    ) -> CommonResult<()> {
        let expected_proof = hashing::create_account_identifier(owner, account.as_ref());
        
        if *proof != expected_proof {
            return Err(CommonError::InsufficientPermissions);
        }
        
        Ok(())
    }

    /// Verify account derivation is correct
    pub fn verify_account_derivation(
        derived: &Pubkey,
        base: &Pubkey,
        seed: &[u8],
        program_id: &Pubkey
    ) -> CommonResult<()> {
        let seeds = [base.as_ref(), seed];
        let (expected, _) = Pubkey::find_program_address(&seeds, program_id);
        
        if *derived != expected {
            return Err(CommonError::AccountValidationFailed);
        }
        
        Ok(())
    }

    /// Validate account signature using deterministic verification
    pub fn validate_deterministic_signature(
        message: &[u8],
        account: &Pubkey,
        signature_data: &[u8]
    ) -> CommonResult<()> {
        // Create expected signature hash
        let mut verification_data = Vec::new();
        verification_data.extend_from_slice(message);
        verification_data.extend_from_slice(account.as_ref());
        
        let expected_hash = hashing::hash_account_data(&verification_data);
        
        if signature_data.len() != 32 {
            return Err(CommonError::Custom("Invalid signature length".to_string()));
        }
        
        let provided_hash: [u8; 32] = signature_data.try_into()
            .map_err(|_| CommonError::Custom("Invalid signature format".to_string()))?;
        
        if expected_hash != provided_hash {
            return Err(CommonError::InsufficientPermissions);
        }
        
        Ok(())
    }
}

/// Utilities for secure account operations
pub mod security {
    use super::*;

    /// Check if an account meets security requirements
    pub fn validate_account_security_level(
        account_data: &[u8],
        required_entropy: u8
    ) -> CommonResult<()> {
        if account_data.len() < 32 {
            return Err(CommonError::AccountValidationFailed);
        }
        
        // Simple entropy check - count unique bytes
        let mut byte_counts = [0u8; 256];
        for &byte in account_data {
            byte_counts[byte as usize] = 1;
        }
        
        let unique_bytes = byte_counts.iter().sum::<u8>();
        
        if unique_bytes < required_entropy {
            return Err(CommonError::Custom(
                format!("Account entropy {} below required {}", unique_bytes, required_entropy)
            ));
        }
        
        Ok(())
    }

    /// Generate a security token for account operations
    pub fn generate_security_token(
        account: &Pubkey,
        operation_type: &str,
        timestamp: i64
    ) -> [u8; 32] {
        let mut token_data = Vec::new();
        token_data.extend_from_slice(account.as_ref());
        token_data.extend_from_slice(operation_type.as_bytes());
        token_data.extend_from_slice(&timestamp.to_le_bytes());
        
        hashing::hash_account_data(&token_data)
    }

    /// Verify a security token is valid for the operation
    pub fn verify_security_token(
        token: &[u8; 32],
        account: &Pubkey,
        operation_type: &str,
        timestamp: i64,
        max_age_seconds: i64
    ) -> CommonResult<()> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| CommonError::Custom("Time error".to_string()))?
            .as_secs() as i64;
        
        if current_time - timestamp > max_age_seconds {
            return Err(CommonError::Custom("Security token expired".to_string()));
        }
        
        let expected_token = generate_security_token(account, operation_type, timestamp);
        
        if *token != expected_token {
            return Err(CommonError::InsufficientPermissions);
        }
        
        Ok(())
    }
} 