use common::{CommonError, CommonResult, constants::DEFAULT_DECIMALS};

/// Mathematical operations for token calculations
pub mod token_math {
    use super::*;

    /// Calculate token amount with decimals
    pub fn calculate_token_amount(base_amount: u64, decimals: u8) -> CommonResult<u64> {
        let multiplier = 10_u64.pow(decimals as u32);
        base_amount.checked_mul(multiplier)
            .ok_or(CommonError::InvalidCalculation)
    }

    /// Calculate token amount using default decimals from common
    pub fn calculate_default_token_amount(base_amount: u64) -> CommonResult<u64> {
        calculate_token_amount(base_amount, DEFAULT_DECIMALS)
    }

    /// Convert token amount back to base units
    pub fn convert_to_base_units(token_amount: u64, decimals: u8) -> CommonResult<u64> {
        let divisor = 10_u64.pow(decimals as u32);
        if divisor == 0 {
            return Err(CommonError::InvalidCalculation);
        }
        Ok(token_amount / divisor)
    }
}

/// Percentage and ratio calculations
pub mod percentage {
    use super::*;

    /// Calculate percentage of an amount
    pub fn calculate_percentage(amount: u64, percentage_basis_points: u16) -> CommonResult<u64> {
        if percentage_basis_points > 10000 {
            return Err(CommonError::InvalidCalculation);
        }
        
        let result = (amount as u128)
            .checked_mul(percentage_basis_points as u128)
            .and_then(|val| val.checked_div(10000))
            .and_then(|val| u64::try_from(val).ok())
            .ok_or(CommonError::InvalidCalculation)?;
            
        Ok(result)
    }

    /// Calculate compound interest
    pub fn calculate_compound_interest(
        principal: u64, 
        rate_basis_points: u16, 
        periods: u32
    ) -> CommonResult<u64> {
        if rate_basis_points > 10000 {
            return Err(CommonError::InvalidCalculation);
        }
        
        let rate = rate_basis_points as f64 / 10000.0;
        let compound_factor = (1.0 + rate).powi(periods as i32);
        let result = (principal as f64 * compound_factor) as u64;
        
        if result < principal {
            return Err(CommonError::InvalidCalculation);
        }
        
        Ok(result)
    }
}

/// Safe arithmetic operations
pub mod safe_math {
    use super::*;

    pub fn safe_add(a: u64, b: u64) -> CommonResult<u64> {
        a.checked_add(b).ok_or(CommonError::InvalidCalculation)
    }

    pub fn safe_sub(a: u64, b: u64) -> CommonResult<u64> {
        a.checked_sub(b).ok_or(CommonError::InvalidCalculation)
    }

    pub fn safe_mul(a: u64, b: u64) -> CommonResult<u64> {
        a.checked_mul(b).ok_or(CommonError::InvalidCalculation)
    }

    pub fn safe_div(a: u64, b: u64) -> CommonResult<u64> {
        if b == 0 {
            return Err(CommonError::InvalidCalculation);
        }
        Ok(a / b)
    }
} 